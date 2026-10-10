use crate::Verdict;
use crate::lookup;
use crate::shell::{Token, literal, may_be_flag, shown, splits};
use std::collections::HashSet;
use std::process::Command;

const GLOBAL_VALUE_FLAGS: [&str; 7] = [
    "-C",
    "-c",
    "--git-dir",
    "--work-tree",
    "--namespace",
    "--exec-path",
    "--super-prefix",
];

const TAG_WRITE_FLAGS: [&str; 17] = [
    "-a",
    "--annotate",
    "-s",
    "--sign",
    "-u",
    "--local-user",
    "-m",
    "--message",
    "-F",
    "--file",
    "-f",
    "--force",
    "-d",
    "--delete",
    "-e",
    "--edit",
    "--cleanup",
];
const TAG_READ_FLAGS: [&str; 17] = [
    "-l",
    "--list",
    "-n",
    "--contains",
    "--no-contains",
    "--points-at",
    "--merged",
    "--no-merged",
    "--sort",
    "--format",
    "--column",
    "--no-column",
    "-i",
    "--ignore-case",
    "--omit-empty",
    "-v",
    "--verify",
];
const PUSH_TAG_FLAGS: [&str; 3] = ["--tags", "--follow-tags", "--mirror"];
const PUSH_VALUE_FLAGS: [&str; 6] = [
    "-o",
    "--push-option",
    "--repo",
    "--receive-pack",
    "--exec",
    "--recurse-submodules",
];
const CONFIG_READ_FLAGS: [&str; 6] = [
    "--get",
    "--get-all",
    "--get-regexp",
    "--get-urlmatch",
    "--list",
    "-l",
];
const CONFIG_READ_ACTIONS: [&str; 2] = ["get", "list"];
const REMOTE_WRITE_ACTIONS: [&str; 9] = [
    "add",
    "rename",
    "remove",
    "rm",
    "set-head",
    "set-branches",
    "set-url",
    "prune",
    "update",
];
const PUSH_SETTINGS: &str = r"^(push\.followtags|remote\..*\.(mirror|push)|branch\..*\.merge)$";
const FALSE_VALUES: [&str; 4] = ["false", "no", "off", ""];
const TAG_REF: &str = "refs/tags/";
const BRANCH_REF: &str = "refs/heads/";

pub struct Push {
    globals: Vec<String>,
    bare: bool,
}

type Invocation<'a> = (&'a [Token], &'a str, &'a [Token]);

fn subcommand(args: &[Token]) -> Result<Option<Invocation<'_>>, &Token> {
    let mut index = 0;
    while index < args.len() {
        let token = &args[index];
        if !literal(token) {
            return Err(token);
        }

        let value = token.value.as_str();
        if GLOBAL_VALUE_FLAGS.contains(&value) {
            if let Some(given) = args.get(index + 1)
                && splits(given)
            {
                return Err(given);
            }
            index += 2;
            continue;
        }
        if value.starts_with('-') {
            index += 1;
            continue;
        }
        return Ok(Some((&args[..index], value, &args[index + 1..])));
    }
    Ok(None)
}

fn refspecs(args: &[Token]) -> Vec<&str> {
    let mut words = Vec::new();
    let mut flags_ended = false;
    let mut rest = args.iter();

    while let Some(token) = rest.next() {
        let value = token.value.as_str();
        if flags_ended || !value.starts_with('-') {
            words.push(value);
        } else if value == "--" {
            flags_ended = true;
        } else if PUSH_VALUE_FLAGS.contains(&value) {
            rest.next();
        }
    }

    words.into_iter().skip(1).collect()
}

fn reaches_a_branch(refspec: &str) -> bool {
    let refspec = refspec.strip_prefix('+').unwrap_or(refspec);
    match refspec.split_once(':') {
        Some((_, destination)) => destination.starts_with(BRANCH_REF),
        None => refspec == "HEAD" || refspec.starts_with(BRANCH_REF),
    }
}

fn action(args: &[Token]) -> Option<&Token> {
    args.iter().find(|token| !token.value.starts_with('-'))
}

fn reads_config(args: &[Token]) -> bool {
    args.iter()
        .any(|token| literal(token) && CONFIG_READ_FLAGS.contains(&token.value.as_str()))
        || action(args).is_some_and(|token| {
            literal(token) && CONFIG_READ_ACTIONS.contains(&token.value.as_str())
        })
}

fn writes_remote(args: &[Token]) -> bool {
    args.iter().any(|token| !literal(token))
        || action(args).is_some_and(|token| REMOTE_WRITE_ACTIONS.contains(&token.value.as_str()))
}

fn on(value: Option<&str>) -> bool {
    let Some(value) = value else {
        return true;
    };
    let value = value.to_lowercase();
    !FALSE_VALUES.contains(&value.as_str()) && value.parse::<i64>() != Ok(0)
}

pub fn judge(push: &Push, directory: Option<&str>) -> Option<Verdict> {
    let mut command = Command::new("git");
    command
        .args(&push.globals)
        .args(["config", "-z", "--get-regexp", PUSH_SETTINGS]);
    if let Some(directory) = directory {
        command.current_dir(directory);
    }

    let found = lookup::run(command).and_then(|answer| match answer.status {
        Some(0 | 1) => Ok(answer.stdout),
        _ => Err(answer.failure()),
    });
    let settings = match found {
        Ok(settings) => settings,
        Err(failure) => {
            return Some(Verdict::Deny(format!(
                "`git push` is let through only after this hook has looked at the git configuration it would run under, and that could not be done: `git config` {failure}."
            )));
        }
    };

    for setting in settings.split('\0').filter(|setting| !setting.is_empty()) {
        let (key, value) = match setting.split_once('\n') {
            Some((key, value)) => (key, Some(value)),
            None => (setting, None),
        };
        let shown = value.unwrap_or_default();

        if key == "push.followtags" && on(value) {
            return Some(Verdict::Deny(
                "`git push` would also send tags, because push.followTags is on in the git configuration, and sending a tag creates it on the remote. Ask the user to push, or to turn push.followTags off."
                    .to_string(),
            ));
        }
        if key.starts_with("remote.") && key.ends_with(".mirror") && on(value) {
            return Some(Verdict::Deny(format!(
                "`git push` could mirror every ref, tags included, because {key} is on in the git configuration. Ask the user to push."
            )));
        }
        if !push.bare {
            continue;
        }
        if key.starts_with("remote.") && key.ends_with(".push") && !reaches_a_branch(shown) {
            return Some(Verdict::Deny(format!(
                "`git push` names nothing to push, and {key} in the git configuration makes that {shown}, which does not surely reach a branch. Name what to push, as in `git push origin HEAD`."
            )));
        }
        if key.starts_with("branch.") && key.ends_with(".merge") && !shown.starts_with(BRANCH_REF) {
            return Some(Verdict::Deny(format!(
                "`git push` names nothing to push, and {key} in the git configuration points at {shown}, which is not a branch. Name what to push, as in `git push origin HEAD`."
            )));
        }
    }

    None
}

fn splitting(name: &str, token: &Token, harm: &str) -> Verdict {
    Verdict::Deny(format!(
        "An argument of `git {name}` contains an unquoted shell expansion or pattern ({}), which can turn into further arguments, so it cannot be shown that {harm}. Quote it, or write it out literally.",
        shown(token)
    ))
}

fn flag_names(args: &[Token]) -> HashSet<&str> {
    let mut names = HashSet::new();
    for token in args {
        let value = token.value.as_str();
        if !value.starts_with('-') {
            continue;
        }
        names.insert(value.split('=').next().unwrap_or(value));
        if let Some(count) = value.strip_prefix("-n")
            && !count.is_empty()
            && count.chars().all(|char| char.is_ascii_digit())
        {
            names.insert("-n");
        }
    }
    names
}

fn matched(flags: &HashSet<&str>, against: &[&str]) -> Option<String> {
    let mut found: Vec<&str> = flags
        .iter()
        .filter(|flag| against.contains(*flag))
        .copied()
        .collect();
    found.sort_unstable();
    if found.is_empty() {
        None
    } else {
        Some(found.join(" "))
    }
}

pub fn check(args: &[Token]) -> Option<Verdict> {
    let (globals, name, rest) = match subcommand(args) {
        Ok(found) => found?,
        Err(token) => {
            return Some(Verdict::Deny(format!(
                "{} comes before the end of the git subcommand and contains a shell expansion or pattern, so which subcommand git would run cannot be determined. Quote it if it is the value of a flag, and write the subcommand out literally.",
                shown(token)
            )));
        }
    };

    if name == "tag" {
        let flags = flag_names(rest);
        if let Some(found) = matched(&flags, &TAG_WRITE_FLAGS) {
            return Some(Verdict::Deny(format!(
                "`git tag` was given {found}, which creates, deletes or rewrites a tag. Listing tags is allowed -- `git tag -l`, `git tag --points-at`, `git describe --tags` all pass -- but making one is for the user to do."
            )));
        }
        if let Some(token) = rest.iter().find(|token| splits(token)) {
            return Some(splitting(name, token, "no tag is created or deleted"));
        }
        if let Some(token) = rest.iter().find(|token| may_be_flag(token)) {
            return Some(Verdict::Deny(format!(
                "An argument of `git tag` ({}) comes out of a shell expansion and could turn out to be a flag that creates or deletes a tag. Give it as --flag=value, start it with literal text, or write it out literally.",
                shown(token)
            )));
        }
        if flags.iter().any(|flag| TAG_READ_FLAGS.contains(flag)) {
            return None;
        }
        if rest.iter().any(|token| !token.value.starts_with('-')) {
            return Some(Verdict::Deny(
                "`git tag` names a tag to create. Listing tags is allowed -- `git tag` with no argument, `git tag -l`, `git describe --tags` -- but creating one is for the user to do."
                    .to_string(),
            ));
        }
        return None;
    }

    if name == "push" {
        if let Some(found) = matched(&flag_names(rest), &PUSH_TAG_FLAGS) {
            return Some(Verdict::Deny(format!(
                "`git push` was given {found}, which sends tags to the remote and so creates them there. Push the branch on its own, and ask the user to push tags."
            )));
        }
        if rest.iter().any(|token| token.value.contains(TAG_REF)) {
            return Some(Verdict::Deny(
                "`git push` names a refs/tags/ refspec, which creates a tag on the remote even when no local tag exists. Push to refs/heads/ instead, and ask the user to push tags."
                    .to_string(),
            ));
        }
        if let Some(token) = rest.iter().find(|token| splits(token)) {
            return Some(splitting(name, token, "no tag is sent to the remote"));
        }
        if let Some(token) = rest.iter().find(|token| !literal(token)) {
            return Some(Verdict::Deny(format!(
                "An argument of `git push` ({}) comes out of a shell expansion, so it could turn out to be --tags or a refs/tags/ refspec, and it cannot be shown that no tag is sent to the remote. Write the remote and the branch out literally, or push HEAD.",
                shown(token)
            )));
        }
        if let Some(refspec) = refspecs(rest)
            .into_iter()
            .find(|refspec| !reaches_a_branch(refspec))
        {
            return Some(Verdict::Deny(format!(
                "`git push` names {refspec}, and a bare name reaches a tag when a tag has that name, so it cannot be shown that no tag is sent to the remote. Push the branch that is checked out with `git push origin HEAD`, or name the branch in full, as in refs/heads/NAME or SOURCE:refs/heads/NAME."
            )));
        }
        if let Some(token) = globals.iter().find(|token| !literal(token)) {
            return Some(Verdict::Deny(format!(
                "{} comes ahead of `git push` and out of a shell expansion, so the git configuration the push would run under cannot be looked up. Write it out literally.",
                shown(token)
            )));
        }
        return Some(Verdict::PassIfPushLeavesTags(Push {
            globals: globals.iter().map(|token| token.value.clone()).collect(),
            bare: refspecs(rest).is_empty(),
        }));
    }

    if name == "config" && !reads_config(rest) {
        return Some(Verdict::Unsettling(
            "changes the git configuration with `git config`",
        ));
    }
    if name == "remote" && writes_remote(rest) {
        return Some(Verdict::Unsettling("changes a remote with `git remote`"));
    }

    if name == "update-ref" {
        if rest.iter().any(|token| token.value.contains(TAG_REF)) {
            return Some(Verdict::Deny(
                "`git update-ref` writes a ref under refs/tags/, which creates a tag without `git tag` being involved. Ask the user to create the tag."
                    .to_string(),
            ));
        }
        if let Some(token) = rest.iter().find(|token| splits(token)) {
            return Some(splitting(name, token, "no ref under refs/tags/ is written"));
        }
        if let Some(token) = rest.iter().find(|token| !literal(token)) {
            return Some(Verdict::Deny(format!(
                "An argument of `git update-ref` ({}) comes out of a shell expansion, so it could turn out to name a ref under refs/tags/, and it cannot be shown that no tag is created. Write the ref out literally.",
                shown(token)
            )));
        }
    }

    None
}

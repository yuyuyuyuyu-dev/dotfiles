use crate::Verdict;
use crate::shell::{Token, literal, may_be_flag, shown, splits};
use std::collections::HashSet;

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
const TAG_REF: &str = "refs/tags/";

fn subcommand(args: &[Token]) -> Result<Option<(&str, &[Token])>, &Token> {
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
        return Ok(Some((value, &args[index + 1..])));
    }
    Ok(None)
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
    let (name, rest) = match subcommand(args) {
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
        return None;
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

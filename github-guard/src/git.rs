use crate::Verdict;
use crate::shell::Token;
use std::collections::HashSet;

// git flags that consume the next token, so the subcommand can be located.
const GLOBAL_VALUE_FLAGS: [&str; 7] = [
    "-C",
    "-c",
    "--git-dir",
    "--work-tree",
    "--namespace",
    "--exec-path",
    "--super-prefix",
];

// `git tag` both lists and writes depending on its flags, which is why tags
// cannot be held by a permission rule: a rule broad enough to stop creation
// also stops listing, and deny rules carry no exceptions.
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

fn subcommand(args: &[Token]) -> Option<(&str, &[Token])> {
    let mut index = 0;
    while index < args.len() {
        let value = args[index].value.as_str();
        if GLOBAL_VALUE_FLAGS.contains(&value) {
            index += 2;
            continue;
        }
        if value.starts_with('-') {
            index += 1;
            continue;
        }
        return Some((value, &args[index + 1..]));
    }
    None
}

fn flag_names(args: &[Token]) -> HashSet<&str> {
    let mut names = HashSet::new();
    for token in args {
        let value = token.value.as_str();
        if !value.starts_with('-') {
            continue;
        }
        names.insert(value.split('=').next().unwrap_or(value));
        // `git tag -n5` is `-n` with a count attached, not a flag of its own.
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
    let (name, rest) = subcommand(args)?;

    if name == "tag" {
        let flags = flag_names(rest);
        if let Some(found) = matched(&flags, &TAG_WRITE_FLAGS) {
            return Some(Verdict::Deny(format!(
                "`git tag` was given {found}, which creates, deletes or rewrites a tag. Listing tags is allowed -- `git tag -l`, `git tag --points-at`, `git describe --tags` all pass -- but making one is for the user to do."
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
        return None;
    }

    if name == "update-ref" && rest.iter().any(|token| token.value.contains(TAG_REF)) {
        return Some(Verdict::Deny(
            "`git update-ref` writes a ref under refs/tags/, which creates a tag without `git tag` being involved. Ask the user to create the tag."
                .to_string(),
        ));
    }

    None
}

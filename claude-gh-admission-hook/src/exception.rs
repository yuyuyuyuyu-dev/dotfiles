use crate::Verdict;
use crate::github::Repository;
use crate::pull_request::PullRequest;
use crate::release::Release;
use crate::shell::{SUBST_PLACEHOLDER, Shape, Token, shape};
use regex::Regex;
use std::sync::LazyLock;

const REPO_FLAGS: [&str; 2] = ["-R", "--repo"];
const DRAFT_FLAGS: [&str; 2] = ["-d", "--draft"];
const TRUE_VALUES: [&str; 6] = ["1", "t", "T", "true", "TRUE", "True"];

static REPOSITORY: LazyLock<Regex> = LazyLock::new(|| {
    Regex::new(r"^([A-Za-z0-9.-]+)/([A-Za-z0-9._-]+)/([A-Za-z0-9._-]+)$").unwrap()
});
static TAG: LazyLock<Regex> = LazyLock::new(|| Regex::new(r"^[A-Za-z0-9._/+@-]+$").unwrap());
static NUMBER: LazyLock<Regex> = LazyLock::new(|| Regex::new(r"^[1-9][0-9]{0,8}$").unwrap());

pub struct Command {
    path: [&'static str; 2],
    value_flags: &'static [&'static str],
    bool_flags: &'static [&'static str],
    judge: fn(&Arguments) -> Verdict,
}

static COMMANDS: [Command; 4] = [
    Command {
        path: ["pr", "create"],
        value_flags: &[
            "-a",
            "--assignee",
            "--attach",
            "-B",
            "--base",
            "-b",
            "--body",
            "-F",
            "--body-file",
            "-H",
            "--head",
            "-l",
            "--label",
            "-m",
            "--milestone",
            "-p",
            "--project",
            "--recover",
            "-r",
            "--reviewer",
            "-T",
            "--template",
            "-t",
            "--title",
            "-R",
            "--repo",
        ],
        bool_flags: &[
            "-d",
            "--draft",
            "--dry-run",
            "-e",
            "--editor",
            "-f",
            "--fill",
            "--fill-first",
            "--fill-verbose",
            "--no-maintainer-edit",
            "-w",
            "--web",
            "-h",
            "--help",
        ],
        judge: created_pull_request,
    },
    Command {
        path: ["pr", "edit"],
        value_flags: &[
            "--add-assignee",
            "--add-label",
            "--add-project",
            "--add-reviewer",
            "--attach",
            "-B",
            "--base",
            "-b",
            "--body",
            "-F",
            "--body-file",
            "-m",
            "--milestone",
            "--remove-assignee",
            "--remove-label",
            "--remove-project",
            "--remove-reviewer",
            "-t",
            "--title",
            "-R",
            "--repo",
        ],
        bool_flags: &["--remove-milestone", "-h", "--help"],
        judge: edited_pull_request,
    },
    Command {
        path: ["release", "create"],
        value_flags: &[
            "--discussion-category",
            "-n",
            "--notes",
            "-F",
            "--notes-file",
            "--notes-start-tag",
            "--target",
            "-t",
            "--title",
            "-R",
            "--repo",
        ],
        bool_flags: &[
            "-d",
            "--draft",
            "--fail-on-no-commits",
            "--generate-notes",
            "--latest",
            "--notes-from-tag",
            "-p",
            "--prerelease",
            "--verify-tag",
            "-h",
            "--help",
        ],
        judge: created_release,
    },
    Command {
        path: ["release", "edit"],
        value_flags: &[
            "--discussion-category",
            "-n",
            "--notes",
            "-F",
            "--notes-file",
            "--tag",
            "--target",
            "-t",
            "--title",
            "-R",
            "--repo",
        ],
        bool_flags: &[
            "--draft",
            "--latest",
            "--prerelease",
            "--verify-tag",
            "-h",
            "--help",
        ],
        judge: edited_release,
    },
];

#[derive(Clone, Copy)]
struct Value<'a> {
    text: &'a str,
    literal: bool,
}

#[derive(Default)]
struct Arguments<'a> {
    draft: Option<bool>,
    repository: Option<Value<'a>>,
    words: Vec<&'a str>,
}

impl<'a> Arguments<'a> {
    fn placed(&self, command: &Command, name: &str) -> Result<(), String> {
        if self.words.len() >= command.path.len() || REPO_FLAGS.contains(&name) {
            return Ok(());
        }
        let named = command.path.join(" ");
        Err(format!(
            "{name} comes before `{named}`, where gh may take the word after it for its value. Put it after `{named}`."
        ))
    }

    fn record(&mut self, name: &str, value: Option<Value<'a>>) {
        if DRAFT_FLAGS.contains(&name) {
            self.draft = Some(value.is_none_or(|value| TRUE_VALUES.contains(&value.text)));
        } else if REPO_FLAGS.contains(&name) {
            self.repository = value;
        }
    }
}

pub fn command(path: &[String]) -> Option<&'static Command> {
    COMMANDS.iter().find(|command| {
        path.len() >= command.path.len()
            && path
                .iter()
                .zip(command.path)
                .all(|(word, expected)| word == expected)
    })
}

pub fn check(command: &Command, args: &[Token], terminator: Option<&Token>) -> Verdict {
    let named = command.path.join(" ");

    let read = match terminator {
        Some(separator) if separator.value.contains(['<', '>']) => Err(format!(
            "a redirection ({}) follows the command, and further arguments could come after it unseen. Run the command without the redirection.",
            separator.value.trim()
        )),
        _ => parse(command, args),
    };

    match read {
        Ok(arguments) if arguments.words.starts_with(&command.path) => (command.judge)(&arguments),
        Ok(_) => Verdict::Deny(format!(
            "`gh {named}` is put to the user only when this hook can read every argument of it, and here it cannot: `{named}` could not be found among them."
        )),
        Err(problem) => Verdict::Deny(format!(
            "`gh {named}` is put to the user only when this hook can read every argument of it, and here it cannot: {problem}"
        )),
    }
}

fn parse<'a>(command: &Command, args: &'a [Token]) -> Result<Arguments<'a>, String> {
    let mut arguments = Arguments::default();
    let mut flags_ended = false;
    let mut rest = args.iter();

    while let Some(token) = rest.next() {
        let word = read(token)?;
        let value = word.text;

        if flags_ended || value == "-" || !value.starts_with('-') {
            if !word.literal {
                return Err(format!(
                    "{} comes out of a shell expansion and is not the value of a flag, so it could turn out to be a flag. Write it out literally.",
                    shown(token)
                ));
            }
            arguments.words.push(value);
            continue;
        }

        if value == "--" {
            flags_ended = true;
            continue;
        }

        if value.starts_with("--") {
            let (name, inline) = match value.split_once('=') {
                Some((name, inline)) => (
                    name,
                    Some(Value {
                        text: inline,
                        ..word
                    }),
                ),
                None => (value, None),
            };
            arguments.placed(command, name)?;
            if command.bool_flags.contains(&name) {
                arguments.record(name, inline);
            } else if command.value_flags.contains(&name) {
                let given = match inline {
                    Some(inline) => inline,
                    None => following(&mut rest, name)?,
                };
                arguments.record(name, Some(given));
            } else {
                return Err(unknown(command, name));
            }
            continue;
        }

        let letters = &value[1..];
        for (offset, letter) in letters.char_indices() {
            let name = format!("-{letter}");
            let after = &letters[offset + letter.len_utf8()..];
            arguments.placed(command, &name)?;
            if command.bool_flags.contains(&name.as_str()) {
                if let Some(inline) = after.strip_prefix('=') {
                    arguments.record(
                        &name,
                        Some(Value {
                            text: inline,
                            ..word
                        }),
                    );
                    break;
                }
                arguments.record(&name, None);
            } else if command.value_flags.contains(&name.as_str()) {
                let given = if after.is_empty() {
                    following(&mut rest, &name)?
                } else {
                    Value {
                        text: after.strip_prefix('=').unwrap_or(after),
                        ..word
                    }
                };
                arguments.record(&name, Some(given));
                break;
            } else {
                return Err(unknown(command, &name));
            }
        }
    }

    Ok(arguments)
}

fn read(token: &Token) -> Result<Value<'_>, String> {
    if token.raw.starts_with('#') {
        return Err(
            "a # follows the command, and what becomes of the text after it depends on the shell. Run the command without the comment."
                .to_string(),
        );
    }
    let literal = match shape(&token.raw) {
        Shape::Literal => true,
        Shape::Opaque => false,
        Shape::Splitting => {
            return Err(format!(
                "{} contains an unquoted shell expansion or pattern, which can turn into several arguments. Quote it, or write the arguments out literally.",
                shown(token)
            ));
        }
    };
    Ok(Value {
        text: &token.value,
        literal,
    })
}

fn following<'a>(rest: &mut std::slice::Iter<'a, Token>, name: &str) -> Result<Value<'a>, String> {
    match rest.next() {
        Some(token) => read(token),
        None => Err(format!("{name} takes a value and none follows it.")),
    }
}

fn unknown(command: &Command, name: &str) -> String {
    format!(
        "{name} is not a flag this hook knows for `gh {}`, so it cannot tell whether the argument after it is that flag's value. If {name} is a real flag, the flag table in claude-gh-admission-hook is what needs the entry.",
        command.path.join(" ")
    )
}

fn shown(token: &Token) -> String {
    token.raw.replace(SUBST_PLACEHOLDER, "$(...)")
}

fn created_pull_request(arguments: &Arguments) -> Verdict {
    if arguments.draft == Some(true) {
        return Verdict::Ask(
            "`gh pr create` would open a draft pull request. That is one of the few writes this hook allows, so it needs the user to approve it rather than being denied."
                .to_string(),
        );
    }
    Verdict::Deny(
        "`gh pr create` would open a pull request that is ready for review, because --draft is not in effect. Only a draft pull request may be created: pass --draft, and leave marking it as ready to the user."
            .to_string(),
    )
}

fn edited_pull_request(arguments: &Arguments) -> Verdict {
    let number = arguments
        .words
        .get(2)
        .filter(|number| NUMBER.is_match(number));
    let Some(number) = number else {
        return Verdict::Deny(
            "`gh pr edit` is only allowed for a draft pull request, and this hook can only look one up by its number: with no number, or with a branch or a URL in its place, it cannot be sure which pull request the command would edit. Name the pull request by its number, as in `gh pr edit 123 -R github.com/OWNER/REPO`."
                .to_string(),
        );
    };
    let Some(repository) = named_repository(arguments) else {
        return Verdict::Deny(
            "`gh pr edit` is only allowed for a draft pull request, and this hook has to look the pull request up in the very repository the command would edit, which it can only be sure of when the command names it in full. Name that repository in full: pass -R HOST/OWNER/REPO, for example -R github.com/OWNER/REPO."
                .to_string(),
        );
    };

    Verdict::AskIfDraftPullRequest(PullRequest {
        repository,
        number: number.to_string(),
    })
}

fn created_release(arguments: &Arguments) -> Verdict {
    if arguments.draft == Some(true) {
        return Verdict::Ask(
            "`gh release create` would save a draft release. That is one of the few writes this hook allows, so it needs the user to approve it rather than being denied."
                .to_string(),
        );
    }
    Verdict::Deny(
        "`gh release create` would publish a release, because --draft is not in effect. Only a draft release may be created: pass --draft, and leave publishing it to the user."
            .to_string(),
    )
}

fn edited_release(arguments: &Arguments) -> Verdict {
    if arguments.draft.is_some() {
        return Verdict::Deny(
            "`gh release edit` was given --draft, which changes whether the release is published. Publishing a draft, or turning a published release back into a draft, is for the user to do. Leave --draft out: a draft release stays a draft when it is edited."
                .to_string(),
        );
    }

    let Some(tag) = arguments.words.get(2) else {
        return Verdict::Deny(
            "`gh release edit` names no release, so there is nothing this hook can show to be a draft. Give the tag of the draft release to edit."
                .to_string(),
        );
    };
    if !TAG.is_match(tag) {
        return Verdict::Deny(format!(
            "`gh release edit` names the release {tag}, and this hook only looks up tags made of letters, digits and the characters . _ / + @ - to find out whether a release is a draft. Ask the user to edit this one."
        ));
    }

    let Some(repository) = named_repository(arguments) else {
        return Verdict::Deny(
            "`gh release edit` is only allowed for a draft release, and this hook has to look the release up in the very repository the command would edit, which it can only be sure of when the command names it in full. Name that repository in full: pass -R HOST/OWNER/REPO, for example -R github.com/OWNER/REPO."
                .to_string(),
        );
    };

    Verdict::AskIfDraftRelease(Release {
        repository,
        tag: tag.to_string(),
    })
}

fn named_repository(arguments: &Arguments) -> Option<Repository> {
    let located = arguments
        .repository
        .filter(|repository| repository.literal)
        .and_then(|repository| REPOSITORY.captures(repository.text))?;
    let (_, [host, owner, name]) = located.extract();

    Some(Repository {
        host: host.to_string(),
        owner: owner.to_string(),
        name: name.to_string(),
    })
}

use crate::Verdict;
use crate::shell::{SUBST_PLACEHOLDER, Shape, Token, shape};

const REPO_FLAGS: [&str; 2] = ["-R", "--repo"];
const DRAFT_FLAGS: [&str; 2] = ["-d", "--draft"];
const TRUE_VALUES: [&str; 6] = ["1", "t", "T", "true", "TRUE", "True"];

pub struct Command {
    path: [&'static str; 2],
    value_flags: &'static [&'static str],
    bool_flags: &'static [&'static str],
    judge: fn(&Arguments) -> Verdict,
}

static COMMANDS: [Command; 1] = [Command {
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
    judge: pull_request,
}];

#[derive(Default)]
struct Arguments<'a> {
    draft: Option<bool>,
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

    fn record(&mut self, name: &str, value: Option<&'a str>) {
        if DRAFT_FLAGS.contains(&name) {
            self.draft = Some(value.is_none_or(|value| TRUE_VALUES.contains(&value)));
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
        let value = read(token)?;

        if flags_ended || value == "-" || !value.starts_with('-') {
            if !matches!(shape(&token.raw), Shape::Literal) {
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
                Some((name, inline)) => (name, Some(inline)),
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
                    arguments.record(&name, Some(inline));
                    break;
                }
                arguments.record(&name, None);
            } else if command.value_flags.contains(&name.as_str()) {
                let given = if after.is_empty() {
                    following(&mut rest, &name)?
                } else {
                    after.strip_prefix('=').unwrap_or(after)
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

fn read(token: &Token) -> Result<&str, String> {
    if token.raw.starts_with('#') {
        return Err(
            "a # follows the command, and what becomes of the text after it depends on the shell. Run the command without the comment."
                .to_string(),
        );
    }
    if matches!(shape(&token.raw), Shape::Splitting) {
        return Err(format!(
            "{} contains an unquoted shell expansion or pattern, which can turn into several arguments. Quote it, or write the arguments out literally.",
            shown(token)
        ));
    }
    Ok(&token.value)
}

fn following<'a>(rest: &mut std::slice::Iter<'a, Token>, name: &str) -> Result<&'a str, String> {
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

fn pull_request(arguments: &Arguments) -> Verdict {
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

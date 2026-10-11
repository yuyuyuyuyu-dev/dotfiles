use crate::shell::{self, Heredoc, Token};
use crate::{Verdict, gh, git, http};
use regex::Regex;
use std::sync::LazyLock;

const KEYWORDS: [&str; 18] = [
    "if", "then", "else", "elif", "fi", "while", "until", "do", "done", "for", "case", "esac",
    "in", "select", "function", "!", "{", "}",
];
const WRAPPERS: [&str; 10] = [
    "command", "builtin", "exec", "env", "nohup", "time", "nice", "stdbuf", "sudo", "doas",
];
const RUNNERS: [&str; 7] = [
    "xargs", "timeout", "watch", "parallel", "ionice", "flock", "retry",
];
const FEEDERS: [&str; 2] = ["xargs", "parallel"];
const MOVERS: [&str; 3] = ["cd", "pushd", "popd"];
const SHELLS: [&str; 5] = ["bash", "sh", "zsh", "dash", "ksh"];
const SHELL_VALUE_FLAGS: [&str; 2] = ["--rcfile", "--init-file"];

static CONTINUATION: LazyLock<Regex> = LazyLock::new(|| Regex::new(r"\\\r?\n").unwrap());
static ASSIGNMENT: LazyLock<Regex> =
    LazyLock::new(|| Regex::new(r"^[A-Za-z_][A-Za-z0-9_]*=").unwrap());

static GIT_FILES: LazyLock<Regex> = LazyLock::new(|| {
    Regex::new(r"(^|[^A-Za-z0-9_-])\.git(/|$|[^A-Za-z0-9_-])|gitconfig|git/config").unwrap()
});
static GIT_VARIABLE: LazyLock<Regex> =
    LazyLock::new(|| Regex::new(r"^GIT_[A-Za-z0-9_]*=").unwrap());

static GH_INVOCATION: LazyLock<Regex> =
    LazyLock::new(|| Regex::new(r"(^|[^A-Za-z0-9_])([^\s;|&()]*/)?gh\s").unwrap());
static GIT_TAG_INVOCATION: LazyLock<Regex> = LazyLock::new(|| {
    Regex::new(
        r"(^|[^A-Za-z0-9_])([^\s;|&()]*/)?git\s[^;|&]*?(\btag\b|--tags\b|--follow-tags\b|--mirror\b|refs/tags/)",
    )
    .unwrap()
});
static HTTP_CLIENT_INVOCATION: LazyLock<Regex> =
    LazyLock::new(|| Regex::new(r"(^|[^A-Za-z0-9_])([^\s;|&()]*/)?(curl|wget)\b").unwrap());

fn mentions_github(text: &str) -> bool {
    GH_INVOCATION.is_match(text)
        || GIT_TAG_INVOCATION.is_match(text)
        || (HTTP_CLIENT_INVOCATION.is_match(text)
            && (http::GH_API_URL.is_match(text) || http::GH_CREDENTIAL.is_match(text)))
}

struct Script<'a> {
    text: &'a str,
    heredocs: &'a [Heredoc],
    github_variables: &'a [String],
    depth: u32,
}

pub fn check_command(text: &str, depth: u32) -> Vec<Verdict> {
    let mut verdicts = Vec::new();
    if depth > 3 {
        if mentions_github(text) {
            verdicts.push(Verdict::Deny(
                "This command mentions gh, a git tag or the GitHub API inside more nested shells and substitutions than this hook reads, so what it would run cannot be determined. Run the GitHub part as a command of its own."
                    .to_string(),
            ));
        }
        return verdicts;
    }

    let text = CONTINUATION.replace_all(text, " ");
    if GIT_FILES.is_match(&text) {
        verdicts.push(Verdict::Unsettling(
            "mentions the .git directory or a git configuration file",
        ));
    }
    let (text, heredocs) = shell::strip_heredocs(&text);
    let github_variables = http::github_variables(&text);
    let (text, inners) = shell::extract_substitutions(&text);

    for inner in &inners {
        verdicts.extend(check_command(inner, depth + 1));
    }

    for heredoc in heredocs.iter().filter(|heredoc| heredoc.expands) {
        for inner in shell::extract_substitutions(&heredoc.body).1 {
            verdicts.extend(check_command(&inner, depth + 1));
        }
    }

    let tokens = match shell::tokenize(&text, true) {
        Ok(tokens) => tokens,
        Err(_) => {
            if mentions_github(&text) {
                verdicts.push(Verdict::Deny(
                    "This command mentions gh, a git tag or the GitHub API, but it could not be tokenized -- an unbalanced quote is the usual cause -- so what it would run cannot be determined. It is denied rather than guessed at. Run the GitHub part as a command of its own."
                        .to_string(),
                ));
            }
            return verdicts;
        }
    };
    let script = Script {
        text: &text,
        heredocs: &heredocs,
        github_variables: &github_variables,
        depth,
    };
    check_tokens(&tokens, &script, &mut verdicts);

    if text.contains('#')
        && let Ok(tokens) = shell::tokenize(&text, false)
    {
        check_tokens(&tokens, &script, &mut verdicts);
    }

    verdicts
}

struct Launch<'a> {
    command: &'a [Token],
    fed: bool,
    direct: bool,
}

fn commands(segment: &[Token]) -> Vec<Launch<'_>> {
    let mut segment = segment;
    while segment.first().is_some_and(|token| {
        ASSIGNMENT.is_match(&token.value) || KEYWORDS.contains(&token.value.as_str())
    }) {
        segment = &segment[1..];
    }

    let launches = segment.first().is_some_and(|head| {
        let name = shell::basename(&head.value);
        WRAPPERS.contains(&name) || RUNNERS.contains(&name)
    });
    let mut found = Vec::new();
    if !launches {
        found.push(Launch {
            command: segment,
            fed: false,
            direct: true,
        });
    }

    for index in 1..segment.len() {
        let name = shell::basename(&segment[index].value);
        let named = match name {
            "git" => true,
            "eval" => launches,
            "gh" => {
                launches
                    || segment
                        .get(index + 1)
                        .is_some_and(|next| gh::names_command(&next.value))
            }
            _ => SHELLS.contains(&name) || http::HTTP_CLIENTS.contains(&name),
        };
        if named {
            found.push(Launch {
                command: &segment[index..],
                fed: segment[..index]
                    .iter()
                    .any(|token| FEEDERS.contains(&shell::basename(&token.value))),
                direct: launches,
            });
        }
    }
    found
}

fn unsettles(segment: &[Token]) -> Option<&'static str> {
    if segment
        .iter()
        .any(|token| GIT_VARIABLE.is_match(&token.value))
    {
        return Some("sets a GIT_ environment variable");
    }

    let head = segment.iter().find(|token| {
        let name = shell::basename(&token.value);
        !ASSIGNMENT.is_match(&token.value)
            && !KEYWORDS.contains(&token.value.as_str())
            && !WRAPPERS.contains(&name)
            && !token.value.starts_with('-')
    })?;
    MOVERS
        .contains(&head.value.as_str())
        .then_some("changes the directory")
}

fn check_gh(args: &[Token]) -> Option<Verdict> {
    let head = args.first()?;
    if head.value == "api" {
        return match gh::check_api(&args[1..]) {
            Ok(verdict) => verdict,
            Err(reason) => Some(Verdict::Deny(format!(
                "The `gh api` invocation could not be parsed ({reason}), so the request it would send cannot be determined, and a request that cannot be read cannot be shown to be a read."
            ))),
        };
    }
    gh::check_subcommand(args)
}

fn check_fed(
    name: &str,
    args: &[Token],
    check: impl Fn(&[Token]) -> Option<Verdict>,
) -> Option<Verdict> {
    let written = check(args);
    if matches!(written, Some(Verdict::Deny(_))) {
        return written;
    }

    let mut extended = args.to_vec();
    extended.push(shell::unseen());
    match check(&extended) {
        Some(Verdict::Deny(_)) => Some(Verdict::Deny(format!(
            "This {name} command is run through xargs or parallel, which adds arguments that this hook cannot see, and they could turn it into a write. Give {name} all of its arguments on the command line itself."
        ))),
        _ => written,
    }
}

enum Source<'a> {
    Inline(&'a Token),
    Input,
    File,
}

fn source(args: &[Token]) -> Source<'_> {
    let mut inline = false;
    let mut input = false;
    let mut rest = args.iter();

    while let Some(token) = rest.next() {
        let value = token.value.as_str();
        if value == "-" {
            return Source::Input;
        }
        if value.starts_with("--") {
            if SHELL_VALUE_FLAGS.contains(&value) {
                rest.next();
            }
            continue;
        }
        if let Some(letters) = value.strip_prefix(['-', '+']) {
            inline |= value.starts_with('-') && letters.contains('c');
            input |= letters.contains('s');
            if letters.ends_with(['o', 'O']) {
                rest.next();
            }
            continue;
        }
        return match (inline, input) {
            (true, _) => Source::Inline(token),
            (false, true) => Source::Input,
            (false, false) => Source::File,
        };
    }
    Source::Input
}

fn check_shell(
    name: &str,
    args: &[Token],
    direct: bool,
    script: &Script,
    verdicts: &mut Vec<Verdict>,
) {
    match source(args) {
        Source::Inline(given) => {
            if shell::may_be_flag(given) && mentions_github(script.text) {
                verdicts.push(Verdict::Deny(format!(
                    "The script given to `{name} -c` comes out of a shell expansion, and this command mentions gh, a git tag or the GitHub API, so what the script would run cannot be determined. Write the script out literally."
                )));
            }
            verdicts.extend(check_command(&given.value, script.depth + 1));
        }
        Source::Input if direct => {
            for heredoc in script.heredocs {
                verdicts.extend(check_command(&heredoc.body, script.depth + 1));
            }
            if mentions_github(script.text) {
                verdicts.push(Verdict::Deny(format!(
                    "`{name}` is given its script on standard input here, and this command mentions gh, a git tag or the GitHub API outside a here-document, so what the script would run cannot be determined. Run the GitHub part as a command of its own."
                )));
            }
        }
        _ => {}
    }
}

fn check_tokens(tokens: &[Token], script: &Script, verdicts: &mut Vec<Verdict>) {
    let found = shell::segments(tokens);
    verdicts.extend(
        found
            .iter()
            .filter_map(|segment| unsettles(segment))
            .map(Verdict::Unsettling),
    );

    for launch in found.iter().flat_map(|segment| commands(segment)) {
        let Some(head) = launch.command.first() else {
            continue;
        };
        let name = shell::basename(&head.value);
        let args = &launch.command[1..];
        let fed = launch.fed;

        if SHELLS.contains(&name) {
            check_shell(name, args, launch.direct, script, verdicts);
            continue;
        }

        if name == "eval" {
            if args.iter().any(shell::may_be_flag) && mentions_github(script.text) {
                verdicts.push(Verdict::Deny(
                    "An argument of `eval` comes out of a shell expansion, and this command mentions gh, a git tag or the GitHub API, so what `eval` would run cannot be determined. Write the command out literally."
                        .to_string(),
                ));
            }
            let joined = args
                .iter()
                .map(|token| token.value.as_str())
                .collect::<Vec<_>>()
                .join(" ");
            verdicts.extend(check_command(&joined, script.depth + 1));
            continue;
        }

        let git = |args: &[Token]| git::check(args, launch.direct);
        let http = |args: &[Token]| http::check(name, args, script.github_variables);
        let found = match name {
            "git" if fed => check_fed(name, args, git),
            "git" => git(args),
            "gh" if fed => check_fed(name, args, check_gh),
            "gh" => check_gh(args),
            _ if !http::HTTP_CLIENTS.contains(&name) => None,
            _ if fed => check_fed(name, args, http),
            _ => http(args),
        };
        verdicts.extend(found);
    }
}

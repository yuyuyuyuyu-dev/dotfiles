use crate::shell::{self, Token};
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
const SHELLS: [&str; 5] = ["bash", "sh", "zsh", "dash", "ksh"];

static CONTINUATION: LazyLock<Regex> = LazyLock::new(|| Regex::new(r"\\\r?\n").unwrap());
static ASSIGNMENT: LazyLock<Regex> =
    LazyLock::new(|| Regex::new(r"^[A-Za-z_][A-Za-z0-9_]*=").unwrap());

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

pub fn check_command(text: &str, depth: u32) -> Vec<Verdict> {
    let mut verdicts = Vec::new();
    if depth > 3 {
        return verdicts;
    }

    let text = CONTINUATION.replace_all(text, " ");
    let (text, expanded_bodies) = shell::strip_heredocs(&text);
    let github_variables = http::github_variables(&text);
    let (text, inners) = shell::extract_substitutions(&text);

    for inner in &inners {
        verdicts.extend(check_command(inner, depth + 1));
    }

    for body in &expanded_bodies {
        for inner in shell::extract_substitutions(body).1 {
            verdicts.extend(check_command(&inner, depth + 1));
        }
    }

    let tokens = match shell::tokenize(&text, true) {
        Ok(tokens) => tokens,
        Err(_) => {
            let targets_github =
                http::GH_API_URL.is_match(&text) || http::GH_CREDENTIAL.is_match(&text);
            if GH_INVOCATION.is_match(&text)
                || GIT_TAG_INVOCATION.is_match(&text)
                || (HTTP_CLIENT_INVOCATION.is_match(&text) && targets_github)
            {
                verdicts.push(Verdict::Deny(
                    "This command mentions gh, a git tag or the GitHub API, but it could not be tokenized -- an unbalanced quote is the usual cause -- so what it would run cannot be determined. It is denied rather than guessed at. Run the GitHub part as a command of its own."
                        .to_string(),
                ));
            }
            return verdicts;
        }
    };
    check_tokens(&tokens, &github_variables, depth, &mut verdicts);

    if text.contains('#')
        && let Ok(tokens) = shell::tokenize(&text, false)
    {
        check_tokens(&tokens, &github_variables, depth, &mut verdicts);
    }

    verdicts
}

fn commands(segment: &[Token]) -> Vec<(&[Token], bool)> {
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
    if !launches {
        return vec![(segment, false)];
    }

    (1..segment.len())
        .filter(|&index| {
            let name = shell::basename(&segment[index].value);
            matches!(name, "gh" | "git" | "eval")
                || SHELLS.contains(&name)
                || http::HTTP_CLIENTS.contains(&name)
        })
        .map(|index| {
            let fed = segment[..index]
                .iter()
                .any(|token| FEEDERS.contains(&shell::basename(&token.value)));
            (&segment[index..], fed)
        })
        .collect()
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

fn check_tokens(
    tokens: &[Token],
    github_variables: &[String],
    depth: u32,
    verdicts: &mut Vec<Verdict>,
) {
    let found = shell::segments(tokens);
    for (command, fed) in found.iter().flat_map(|segment| commands(segment)) {
        let Some(head) = command.first() else {
            continue;
        };
        let name = shell::basename(&head.value);
        let args = &command[1..];

        if SHELLS.contains(&name) {
            for (index, token) in args.iter().enumerate() {
                if token.value == "-c"
                    && let Some(script) = args.get(index + 1)
                {
                    verdicts.extend(check_command(&script.value, depth + 1));
                }
            }
            continue;
        }

        if name == "eval" {
            let script = args
                .iter()
                .map(|token| token.value.as_str())
                .collect::<Vec<_>>()
                .join(" ");
            verdicts.extend(check_command(&script, depth + 1));
            continue;
        }

        let found = match name {
            "git" if fed => check_fed(name, args, git::check),
            "git" => git::check(args),
            "gh" if fed => check_fed(name, args, check_gh),
            "gh" => check_gh(args),
            _ if !http::HTTP_CLIENTS.contains(&name) => None,
            _ if fed => check_fed(name, args, |args| http::check(name, args, github_variables)),
            _ => http::check(name, args, github_variables),
        };
        verdicts.extend(found);
    }
}

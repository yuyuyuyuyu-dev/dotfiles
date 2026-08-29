use crate::shell;
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
const SHELLS: [&str; 5] = ["bash", "sh", "zsh", "dash", "ksh"];

static CONTINUATION: LazyLock<Regex> = LazyLock::new(|| Regex::new(r"\\\r?\n").unwrap());
static ASSIGNMENT: LazyLock<Regex> =
    LazyLock::new(|| Regex::new(r"^[A-Za-z_][A-Za-z0-9_]*=").unwrap());

// Every gh invocation matters now that the allow-list decides, not a list of
// known-bad subcommands.
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

pub fn check_command(text: &str, depth: u32) -> Option<Verdict> {
    if depth > 3 {
        return None;
    }

    let text = CONTINUATION.replace_all(text, " ");
    let (text, expanded_bodies) = shell::strip_heredocs(&text);
    let github_variables = http::github_variables(&text);
    let (text, inners) = shell::extract_substitutions(&text);

    for inner in &inners {
        if let Some(verdict) = check_command(inner, depth + 1) {
            return Some(verdict);
        }
    }

    // An unquoted heredoc body is text, not a command list, so only the parts the
    // shell actually runs are followed.
    for body in &expanded_bodies {
        for inner in shell::extract_substitutions(body).1 {
            if let Some(verdict) = check_command(&inner, depth + 1) {
                return Some(verdict);
            }
        }
    }

    let tokens = match shell::tokenize(&text) {
        Ok(tokens) => tokens,
        Err(_) => {
            let targets_github =
                http::GH_API_URL.is_match(&text) || http::GH_CREDENTIAL.is_match(&text);
            if GH_INVOCATION.is_match(&text)
                || GIT_TAG_INVOCATION.is_match(&text)
                || (HTTP_CLIENT_INVOCATION.is_match(&text) && targets_github)
            {
                return Some(Verdict::Deny(
                    "This command mentions gh, a git tag or the GitHub API, but it could not be tokenized -- an unbalanced quote is the usual cause -- so what it would run cannot be determined. It is denied rather than guessed at. Run the GitHub part as a command of its own."
                        .to_string(),
                ));
            }
            return None;
        }
    };

    for segment in shell::segments(&tokens) {
        let mut segment = segment;
        while segment.first().is_some_and(|token| {
            ASSIGNMENT.is_match(&token.value)
                || KEYWORDS.contains(&token.value.as_str())
                || WRAPPERS.contains(&shell::basename(&token.value))
        }) {
            segment = &segment[1..];
        }
        let Some(head) = segment.first() else {
            continue;
        };

        let mut name = shell::basename(&head.value);
        let mut args = &segment[1..];

        if SHELLS.contains(&name) {
            for (index, token) in args.iter().enumerate() {
                if token.value == "-c"
                    && let Some(script) = args.get(index + 1)
                    && let Some(nested) = check_command(&script.value, depth + 1)
                {
                    return Some(nested);
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
            if let Some(nested) = check_command(&script, depth + 1) {
                return Some(nested);
            }
            continue;
        }

        if RUNNERS.contains(&name)
            && let Some(index) = args
                .iter()
                .position(|token| matches!(shell::basename(&token.value), "gh" | "git"))
        {
            segment = &segment[index + 1..];
            name = shell::basename(&segment[0].value);
            args = &segment[1..];
        }

        if name == "git" {
            if let Some(verdict) = git::check(args) {
                return Some(verdict);
            }
            continue;
        }

        if http::HTTP_CLIENTS.contains(&name) {
            if let Some(verdict) = http::check(name, args, &github_variables) {
                return Some(verdict);
            }
            continue;
        }

        if name != "gh" || args.is_empty() {
            continue;
        }

        let verdict = if args[0].value == "api" {
            match gh::check_api(&args[1..]) {
                Ok(verdict) => verdict,
                Err(reason) => Some(Verdict::Deny(format!(
                    "The `gh api` invocation could not be parsed ({reason}), so the request it would send cannot be determined, and a request that cannot be read cannot be shown to be a read."
                ))),
            }
        } else {
            gh::check_subcommand(args)
        };
        if let Some(verdict) = verdict {
            return Some(verdict);
        }
    }

    None
}

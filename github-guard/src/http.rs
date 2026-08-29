use crate::Verdict;
use crate::shell::{Token, has_expansion};
use regex::Regex;
use std::sync::LazyLock;

const READ_METHODS: [&str; 2] = ["GET", "HEAD"];

pub const HTTP_CLIENTS: [&str; 2] = ["curl", "wget"];

pub static GH_API_URL: LazyLock<Regex> = LazyLock::new(|| {
    Regex::new(r"(?i)\b(?:api|uploads)\.github\.com\b|/api/v3/|/api/graphql\b").unwrap()
});
pub static GH_CREDENTIAL: LazyLock<Regex> = LazyLock::new(|| {
    Regex::new(r"(?i)gh\s+auth\s+token|GH_TOKEN|GITHUB_TOKEN|GH_ENTERPRISE_TOKEN").unwrap()
});
static ASSIGNMENT_VALUE: LazyLock<Regex> = LazyLock::new(|| {
    Regex::new(
        r#"(?:^|[;&|(]|\s)([A-Za-z_][A-Za-z0-9_]*)=((?:\$\([^)]*\)|"[^"]*"|'[^']*'|[^\s;&|]*))"#,
    )
    .unwrap()
});

// curl and wget name these differently, but either way the flag means a request
// body, and a request body means the method is not a read.
const BODY_FLAGS: [&str; 14] = [
    "-d",
    "--data",
    "--data-raw",
    "--data-ascii",
    "--data-binary",
    "--data-urlencode",
    "--json",
    "-F",
    "--form",
    "--form-string",
    "--post-data",
    "--post-file",
    "--body-data",
    "--body-file",
];
const UPLOAD_FLAGS: [&str; 2] = ["-T", "--upload-file"];

/// Names of shell variables whose value points at the GitHub API or carries a
/// token, so that a later `$NAME` in a URL is recognised as targeting GitHub.
pub fn github_variables(text: &str) -> Vec<String> {
    let mut names = Vec::new();
    for captures in ASSIGNMENT_VALUE.captures_iter(text) {
        let name = &captures[1];
        let value = &captures[2];
        if (GH_API_URL.is_match(value) || GH_CREDENTIAL.is_match(value))
            && !names.iter().any(|known| known == name)
        {
            names.push(name.to_string());
        }
    }
    names
}

#[derive(Default)]
struct Request {
    method: Option<String>,
    method_raw: Option<String>,
    head: bool,
    force_get: bool,
    upload: bool,
    body: bool,
}

/// Read a bundle such as `-sSLX POST`, where the flags share one dash.
/// Returns whether the method is the next token instead of part of this one.
fn read_short_bundle(value: &str, raw: &str, state: &mut Request) -> bool {
    let letters: Vec<char> = value[1..].chars().collect();
    for (position, letter) in letters.iter().enumerate() {
        match letter {
            'X' => {
                let rest: String = letters[position + 1..].iter().collect();
                if rest.is_empty() {
                    return true;
                }
                state.method = Some(rest);
                state.method_raw = Some(raw.to_string());
                return false;
            }
            'I' => state.head = true,
            'G' => state.force_get = true,
            'T' => {
                state.upload = true;
                return false;
            }
            'd' | 'F' => {
                state.body = true;
                return false;
            }
            _ => {}
        }
    }
    false
}

pub fn check(name: &str, args: &[Token], github_variables: &[String]) -> Option<Verdict> {
    let variables: Vec<Regex> = github_variables
        .iter()
        .map(|variable| {
            Regex::new(&format!(r"\$\{{?{}\b", regex::escape(variable))).expect("escaped name")
        })
        .collect();

    let mut state = Request::default();
    let mut targets_github = false;
    let mut take_method_next = false;

    for token in args {
        let value = token.value.as_str();

        if take_method_next {
            state.method = Some(value.to_string());
            state.method_raw = Some(token.raw.clone());
            take_method_next = false;
            continue;
        }

        if GH_API_URL.is_match(value)
            || GH_CREDENTIAL.is_match(value)
            || variables.iter().any(|variable| variable.is_match(value))
        {
            targets_github = true;
        }

        if value == "-X" || value == "--request" || value == "--method" {
            take_method_next = true;
            continue;
        }
        if let Some(inline) = value
            .strip_prefix("--request=")
            .or_else(|| value.strip_prefix("--method="))
        {
            state.method = Some(inline.to_string());
            state.method_raw = Some(token.raw.clone());
            continue;
        }
        if value == "-I" || value == "--head" {
            state.head = true;
            continue;
        }
        if value == "-G" || value == "--get" {
            state.force_get = true;
            continue;
        }
        if UPLOAD_FLAGS.contains(&value) {
            state.upload = true;
            continue;
        }
        if BODY_FLAGS.contains(&value) {
            state.body = true;
            continue;
        }
        if value
            .split_once('=')
            .is_some_and(|(flag, _)| BODY_FLAGS.contains(&flag))
        {
            state.body = true;
            continue;
        }

        if value.len() > 1 && value.starts_with('-') && !value.starts_with("--") {
            take_method_next = read_short_bundle(value, &token.raw, &mut state);
        }
    }

    if !targets_github {
        return None;
    }

    if let Some(raw) = state.method_raw.as_deref()
        && has_expansion(raw)
    {
        return Some(Verdict::Deny(format!(
            "The HTTP method given to `{name}` is a shell expansion ({raw}), so it cannot be shown to be GET or HEAD. Write the method out literally."
        )));
    }

    let effective = match &state.method {
        Some(method) => method.to_uppercase(),
        None if state.head => "HEAD".to_string(),
        None if state.force_get => "GET".to_string(),
        None if state.upload => "PUT".to_string(),
        None if state.body => "POST".to_string(),
        None => "GET".to_string(),
    };

    if READ_METHODS.contains(&effective.as_str()) {
        return None;
    }

    Some(Verdict::Deny(format!(
        "`{name}` would send {effective} to the GitHub API, which can change repository state. Only GET and HEAD are allowed here. Note that a body flag such as -d, -F or -T sets the method even when -X is absent."
    )))
}

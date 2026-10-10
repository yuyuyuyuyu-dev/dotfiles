use crate::Verdict;
use crate::shell::{Token, has_expansion, may_be_flag, shown, splits};
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

const METHOD_FLAGS: [&str; 3] = ["-X", "--request", "--method"];
const CURL_VALUE_FLAGS: [&str; 22] = [
    "--header",
    "--output",
    "--user",
    "--user-agent",
    "--referer",
    "--cookie",
    "--cookie-jar",
    "--write-out",
    "--max-time",
    "--connect-timeout",
    "--retry",
    "--retry-delay",
    "--retry-max-time",
    "--proxy",
    "--url",
    "--cacert",
    "--cert",
    "--key",
    "--range",
    "--resolve",
    "--dump-header",
    "--stderr",
];
const WGET_VALUE_FLAGS: [&str; 10] = [
    "--header",
    "--output-document",
    "--user-agent",
    "--user",
    "--password",
    "--tries",
    "--timeout",
    "--directory-prefix",
    "--output-file",
    "--wait",
];
const CURL_VALUE_LETTERS: &str = "HouAebcwmxrDEyYzKCXdFT";
const WGET_VALUE_LETTERS: &str = "OUtTPoawQADe";

fn takes_next_word(name: &str, flag: &str) -> bool {
    let (words, letters): (&[&str], &str) = if name == "wget" {
        (&WGET_VALUE_FLAGS, WGET_VALUE_LETTERS)
    } else {
        (&CURL_VALUE_FLAGS, CURL_VALUE_LETTERS)
    };

    if flag.starts_with("--") {
        return words.contains(&flag)
            || METHOD_FLAGS.contains(&flag)
            || BODY_FLAGS.contains(&flag)
            || UPLOAD_FLAGS.contains(&flag);
    }
    let Some(bundle) = flag.strip_prefix('-') else {
        return false;
    };
    match bundle
        .char_indices()
        .find(|(_, letter)| letters.contains(*letter))
    {
        Some((offset, letter)) => offset + letter.len_utf8() == bundle.len(),
        None => false,
    }
}

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
    splitting: Option<String>,
    flag_like: Option<String>,
}

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
    let mut value_expected = false;

    for token in args {
        let value = token.value.as_str();

        if splits(token) {
            state.splitting.get_or_insert_with(|| shown(token));
        } else if !value_expected && may_be_flag(token) {
            state.flag_like.get_or_insert_with(|| shown(token));
        }
        value_expected = !value_expected && takes_next_word(name, value);

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

    if let Some(word) = state.splitting.as_deref() {
        return Some(Verdict::Deny(format!(
            "An argument of `{name}` contains an unquoted shell expansion or pattern ({word}), which can turn into further arguments, so the request that would be sent to the GitHub API cannot be determined. Quote it, or write the request out with literal values."
        )));
    }
    if let Some(word) = state.flag_like.as_deref() {
        return Some(Verdict::Deny(format!(
            "An argument of `{name}` ({word}) comes out of a shell expansion and is not the value of a flag this hook knows, so it could turn out to be a flag that changes the request sent to the GitHub API. Start it with literal text, or write it out literally."
        )));
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

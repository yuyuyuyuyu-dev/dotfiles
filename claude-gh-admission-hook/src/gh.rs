use crate::shell::{
    SUBST_PLACEHOLDER, Shape, Token, Unanalyzable, has_expansion, literal, shape, shown,
};
use crate::{Verdict, exception};
use regex::Regex;
use std::sync::LazyLock;

const READ_METHODS: [&str; 2] = ["GET", "HEAD"];

const VALUE_FLAGS: [&str; 15] = [
    "-X",
    "--method",
    "-f",
    "--raw-field",
    "-F",
    "--field",
    "-H",
    "--header",
    "--hostname",
    "--input",
    "-q",
    "--jq",
    "-t",
    "--template",
    "--cache",
];
const BOOL_FLAGS: [&str; 8] = [
    "-i",
    "--include",
    "--paginate",
    "--silent",
    "--slurp",
    "--verbose",
    "-h",
    "--help",
];
const FIELD_FLAGS: [&str; 4] = ["-f", "--raw-field", "-F", "--field"];

const GH_FLAGS_WITHOUT_VALUE: [&str; 2] = ["--help", "--version"];

const READ_GH_COMMANDS: &[&[&str]] = &[
    &["browse"],
    &["status"],
    &["licenses"],
    &["completion"],
    &["help"],
    &["version"],
    &["auth", "status"],
    &["pr", "list"],
    &["pr", "status"],
    &["pr", "checks"],
    &["pr", "diff"],
    &["pr", "view"],
    &["pr", "checkout"],
    &["issue", "list"],
    &["issue", "status"],
    &["issue", "view"],
    &["repo", "list"],
    &["repo", "view"],
    &["repo", "clone"],
    &["repo", "gitignore"],
    &["repo", "license"],
    &["repo", "read-dir"],
    &["repo", "read-file"],
    &["repo", "deploy-key", "list"],
    &["repo", "autolink", "list"],
    &["repo", "autolink", "view"],
    &["release", "list"],
    &["release", "view"],
    &["release", "download"],
    &["release", "verify"],
    &["release", "verify-asset"],
    &["run", "list"],
    &["run", "view"],
    &["run", "watch"],
    &["run", "download"],
    &["workflow", "list"],
    &["workflow", "view"],
    &["cache", "list"],
    &["label", "list"],
    &["ruleset", "check"],
    &["ruleset", "list"],
    &["ruleset", "view"],
    &["secret", "list"],
    &["variable", "list"],
    &["variable", "get"],
    &["org", "list"],
    &["project", "list"],
    &["project", "view"],
    &["project", "field-list"],
    &["project", "item-list"],
    &["gist", "list"],
    &["gist", "view"],
    &["gist", "clone"],
    &["search", "code"],
    &["search", "commits"],
    &["search", "issues"],
    &["search", "prs"],
    &["search", "repos"],
    &["attestation", "download"],
    &["attestation", "trusted-root"],
    &["attestation", "verify"],
    &["gpg-key", "list"],
    &["ssh-key", "list"],
    &["config", "get"],
    &["config", "list"],
    &["alias", "list"],
    &["extension", "list"],
    &["extension", "search"],
    &["extension", "browse"],
    &["codespace", "list"],
    &["codespace", "view"],
    &["codespace", "logs"],
    &["discussion", "list"],
    &["discussion", "view"],
    &["skill", "list"],
    &["skill", "search"],
    &["skill", "preview"],
    &["agent-task", "list"],
    &["agent-task", "view"],
];

const BARE_READ_GH_COMMANDS: &[&[&str]] = &[
    &["auth"],
    &["pr"],
    &["issue"],
    &["repo"],
    &["release"],
    &["run"],
    &["workflow"],
    &["cache"],
    &["label"],
    &["ruleset"],
    &["secret"],
    &["variable"],
    &["org"],
    &["project"],
    &["gist"],
    &["search"],
    &["attestation"],
    &["gpg-key"],
    &["ssh-key"],
    &["config"],
    &["alias"],
    &["extension"],
    &["codespace"],
    &["discussion"],
    &["skill"],
    &["agent-task"],
    &["codespace", "ports"],
];

const GH_ALIASES: [(&str, [&str; 2]); 1] = [("co", ["pr", "checkout"])];

static MUTATION: LazyLock<Regex> = LazyLock::new(|| Regex::new(r"(?i)\bmutation\b").unwrap());
static SCHEME: LazyLock<Regex> =
    LazyLock::new(|| Regex::new(r"^[A-Za-z][A-Za-z0-9+.\-]*://[^/]*").unwrap());
static API_PREFIX: LazyLock<Regex> = LazyLock::new(|| Regex::new(r"^api/(v3/)?").unwrap());

fn listed(list: &[&[&str]], path: &[String]) -> bool {
    list.iter()
        .any(|entry| entry.len() == path.len() && entry.iter().zip(path).all(|(a, b)| a == b))
}

fn takes_next_word(flag: &str) -> bool {
    if flag.contains('=') || GH_FLAGS_WITHOUT_VALUE.contains(&flag) {
        return false;
    }
    match flag.strip_prefix("--") {
        Some(name) => !name.is_empty(),
        None => flag.starts_with('-') && flag.len() == 2,
    }
}

fn undeterminable(token: &Token) -> Verdict {
    Verdict::Deny(format!(
        "{} comes before the end of the gh subcommand and contains a shell expansion or pattern, so which subcommand gh would run cannot be determined. Write the subcommand out literally, and put the flags after it.",
        shown(token)
    ))
}

pub fn check_subcommand(args: &[Token]) -> Option<Verdict> {
    let mut path: Vec<String> = Vec::new();
    let mut rest = args.iter();

    while let Some(token) = rest.next() {
        if !literal(token) {
            return Some(undeterminable(token));
        }

        let value = token.value.as_str();
        if takes_next_word(value) {
            if let Some(skipped) = rest.next()
                && matches!(shape(&skipped.raw), Shape::Splitting)
            {
                return Some(undeterminable(skipped));
            }
            continue;
        }
        if value != "--" && (value.is_empty() || value.starts_with('-')) {
            continue;
        }

        path.push(value.to_string());
        if let Some((_, target)) = GH_ALIASES.iter().find(|(name, _)| path == [*name]) {
            path = target.iter().map(|part| part.to_string()).collect();
        }

        if let Some(command) = exception::command(&path) {
            return Some(exception::check(command, args));
        }
        if listed(READ_GH_COMMANDS, &path) {
            return None;
        }
        if path.len() == 3 {
            break;
        }
    }

    if path.is_empty() || listed(BARE_READ_GH_COMMANDS, &path) {
        return None;
    }

    let group = path.len().min(2);
    let named = if listed(BARE_READ_GH_COMMANDS, &path[..group]) {
        path.join(" ")
    } else {
        path[..group].join(" ")
    };
    Some(Verdict::Deny(format!(
        "`gh {named}` is not on this hook's allow-list of gh subcommands that only read from GitHub, so it cannot be shown to leave GitHub unchanged. Read the data with a gh command that is on the list, or ask the user to run this one. If `gh {named}` really only reads, the allow-list in claude-gh-admission-hook is what needs the entry."
    )))
}

struct Api {
    method: Option<String>,
    method_raw: Option<String>,
    endpoint: Option<String>,
    body_params: bool,
    queries: Vec<String>,
    opaque_query: bool,
    expanded_query: bool,
    flag_like: Option<String>,
}

fn collect(pair: &str, token: &Token, api: &mut Api) {
    let (key, value) = pair.split_once('=').unwrap_or((pair, ""));
    if !literal(token)
        && (has_expansion(key) || key.contains(SUBST_PLACEHOLDER) || key.trim() == "query")
    {
        api.expanded_query = true;
        return;
    }
    if key.trim() != "query" {
        return;
    }
    if value.starts_with('@') {
        api.opaque_query = true;
        return;
    }
    api.queries.push(value.to_string());
}

fn parse_api(args: &[Token]) -> Result<Api, Unanalyzable> {
    let mut api = Api {
        method: None,
        method_raw: None,
        endpoint: None,
        body_params: false,
        queries: Vec::new(),
        opaque_query: false,
        expanded_query: false,
        flag_like: None,
    };

    let mut index = 0;
    while index < args.len() {
        let token = &args[index];
        let value = token.value.as_str();

        if value == "--" {
            for rest in &args[index + 1..] {
                if api.endpoint.is_none() {
                    api.endpoint = Some(rest.value.clone());
                }
            }
            break;
        }

        if value.starts_with("--")
            && let Some((name, inline)) = value.split_once('=')
        {
            if name == "--method" {
                api.method = Some(inline.to_string());
                api.method_raw = Some(token.raw.clone());
            } else if FIELD_FLAGS.contains(&name) {
                api.body_params = true;
                collect(inline, token, &mut api);
            } else if name == "--input" {
                api.body_params = true;
                api.opaque_query = true;
            } else if !VALUE_FLAGS.contains(&name) && !BOOL_FLAGS.contains(&name) {
                return Err(Unanalyzable(format!("unknown flag {name}")));
            }
            index += 1;
            continue;
        }

        if VALUE_FLAGS.contains(&value) {
            let Some(next) = args.get(index + 1) else {
                return Err(Unanalyzable(format!("flag without value: {value}")));
            };
            if value == "-X" || value == "--method" {
                api.method = Some(next.value.clone());
                api.method_raw = Some(next.raw.clone());
            } else if FIELD_FLAGS.contains(&value) {
                api.body_params = true;
                collect(&next.value, next, &mut api);
            } else if value == "--input" {
                api.body_params = true;
                api.opaque_query = true;
            }
            index += 2;
            continue;
        }

        if BOOL_FLAGS.contains(&value) {
            index += 1;
            continue;
        }

        if let Some(inline) = value.strip_prefix("-X")
            && !inline.is_empty()
        {
            api.method = Some(inline.to_string());
            api.method_raw = Some(token.raw.clone());
            index += 1;
            continue;
        }

        if value.len() > 2 && (value.starts_with("-f") || value.starts_with("-F")) {
            api.body_params = true;
            collect(&value[2..], token, &mut api);
            index += 1;
            continue;
        }

        if value.len() > 2 && value.starts_with("-H") {
            index += 1;
            continue;
        }

        if value.starts_with('-') {
            return Err(Unanalyzable(format!("unknown flag {value}")));
        }

        if !literal(token)
            && (has_expansion(value) || value.contains(SUBST_PLACEHOLDER))
            && !value.starts_with(|first: char| first.is_ascii_alphanumeric() || first == '/')
        {
            api.flag_like.get_or_insert_with(|| shown(token));
        }
        if api.endpoint.is_none() {
            api.endpoint = Some(value.to_string());
        }
        index += 1;
    }

    Ok(api)
}

fn normalize_endpoint(endpoint: &str) -> String {
    let value = SCHEME.replace(endpoint.trim(), "");
    let value = value.split('?').next().unwrap_or_default();
    let value = value.split('#').next().unwrap_or_default();
    let value = value.trim_start_matches('/');
    let value = API_PREFIX.replace(value, "");
    value.trim_end_matches('/').to_lowercase()
}

pub fn check_api(args: &[Token]) -> Result<Option<Verdict>, Unanalyzable> {
    for token in args {
        if matches!(shape(&token.raw), Shape::Splitting) {
            return Ok(Some(Verdict::Deny(format!(
                "An argument of `gh api` contains an unquoted shell expansion or pattern ({}), which can turn into further arguments, so the request that would actually be sent cannot be determined. Quote it, or write the request out with literal values.",
                shown(token)
            ))));
        }
    }

    let api = parse_api(args)?;

    if let Some(word) = api.flag_like.as_deref() {
        return Ok(Some(Verdict::Deny(format!(
            "An argument of `gh api` ({word}) comes out of a shell expansion and is not the value of a flag, so it could turn out to be a flag that changes the request. Start it with literal text, or write it out literally."
        ))));
    }

    let Some(endpoint) = api.endpoint.as_deref() else {
        return Err(Unanalyzable("no endpoint".to_string()));
    };

    if normalize_endpoint(endpoint) == "graphql" {
        if api.opaque_query {
            return Ok(Some(Verdict::Deny(
                "The query body of `gh api graphql` is read from a file or stdin, so it cannot be shown to be free of mutations. Pass the query inline, as -f query='query { ... }', and it can be checked."
                    .to_string(),
            )));
        }
        if api.expanded_query {
            return Ok(Some(Verdict::Deny(
                "A field of `gh api graphql` comes out of a shell expansion and could be the query, so the query cannot be shown to be free of mutations. Write the query out literally, and pass what varies as a GraphQL variable with -F."
                    .to_string(),
            )));
        }
        if api.queries.is_empty() {
            return Err(Unanalyzable("graphql without inline query".to_string()));
        }
        for query in &api.queries {
            if MUTATION.is_match(query) {
                return Ok(Some(Verdict::Deny(
                    "The query passed to `gh api graphql` contains a mutation, which changes GitHub state. Send a query instead, or ask the user to run the mutation."
                        .to_string(),
                )));
            }
        }
        return Ok(None);
    }

    if let Some(raw) = api.method_raw.as_deref()
        && has_expansion(raw)
    {
        return Ok(Some(Verdict::Deny(format!(
            "The method given to `gh api` is a shell expansion ({raw}), so it cannot be shown to be GET or HEAD. Write the method out literally."
        ))));
    }

    let effective = match api.method.as_deref() {
        Some(method) => method.to_uppercase(),
        None if api.body_params => "POST".to_string(),
        None => "GET".to_string(),
    };
    if READ_METHODS.contains(&effective.as_str()) {
        return Ok(None);
    }

    if api.method.is_none() {
        return Ok(Some(Verdict::Deny(
            "`gh api` was given -f/-F/--input with no method, and gh sends POST rather than GET as soon as it has fields, so this would write. Add --method GET if the fields are query parameters of a read."
                .to_string(),
        )));
    }
    Ok(Some(Verdict::Deny(format!(
        "`gh api` would send {effective}, which is not a read and can change GitHub state. Only GET and HEAD are allowed here."
    ))))
}

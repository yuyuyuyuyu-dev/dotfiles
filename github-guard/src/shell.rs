use regex::Regex;
use std::sync::LazyLock;

pub const SUBST_PLACEHOLDER: &str = "__SUBST__";

// shlex emits a run of adjacent punctuation as one token (");", "&&", ")&&"),
// so separators are recognised by content rather than by an exact list.
const PUNCTUATION: [char; 8] = ['(', ')', ';', '<', '>', '|', '&', '\n'];

static HEREDOC: LazyLock<Regex> = LazyLock::new(|| {
    Regex::new(r#"<<-?\s*(?:'([^']*)'|"([^"]*)"|\\?([A-Za-z_][A-Za-z0-9_]*))"#).unwrap()
});

/// A word the shell would pass on, in both the form the command receives and the
/// form it was written in. The written form is what says whether an expansion is
/// live, which the unquoted value can no longer show.
pub struct Token {
    pub value: String,
    pub raw: String,
    pub separator: bool,
}

pub struct Unanalyzable(pub String);

impl std::fmt::Display for Unanalyzable {
    fn fmt(&self, out: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        out.write_str(&self.0)
    }
}

/// Drop heredoc bodies, which the lexer has no concept of.
///
/// A body is data, so a quote or paren inside it otherwise unbalances the whole
/// command and the analyzer denies a command it simply could not read. Bodies of
/// unquoted heredocs are returned separately because the shell still expands
/// them, so a command substitution in one is live.
pub fn strip_heredocs(text: &str) -> (String, Vec<String>) {
    let lines: Vec<&str> = text.split('\n').collect();
    let mut kept: Vec<&str> = Vec::new();
    let mut expanded: Vec<String> = Vec::new();
    let mut index = 0;

    while index < lines.len() {
        let line = lines[index];
        kept.push(line);
        index += 1;

        for captures in HEREDOC.captures_iter(line) {
            let quoted = captures.get(1).or_else(|| captures.get(2));
            // An empty delimiter is no delimiter, so the lines stay where they are
            // rather than being matched against the first blank line below.
            let delimiter = [captures.get(1), captures.get(2), captures.get(3)]
                .into_iter()
                .flatten()
                .map(|group| group.as_str())
                .find(|group| !group.is_empty());
            let Some(delimiter) = delimiter else {
                continue;
            };

            let mut end = index;
            while end < lines.len() && lines[end].trim() != delimiter {
                end += 1;
            }
            // No terminator means this was not a heredoc after all, so leaving the
            // lines in place is safer than swallowing the rest of the command.
            if end >= lines.len() {
                continue;
            }
            if quoted.is_none() {
                expanded.push(lines[index..end].join("\n"));
            }
            index = end + 1;
        }
    }

    (kept.join("\n"), expanded)
}

/// Replace $(...) and `...` with a placeholder, returning their contents.
///
/// The contents are commands in their own right; leaving them inline hides them
/// from every check below, since the lexer reports the whole substitution as one
/// ordinary argument token.
pub fn extract_substitutions(text: &str) -> (String, Vec<String>) {
    let chars: Vec<char> = text.chars().collect();
    let mut out = String::new();
    let mut inners: Vec<String> = Vec::new();
    let mut quote: Option<char> = None;
    let mut index = 0;

    while index < chars.len() {
        let char = chars[index];

        if char == '\\' && index + 1 < chars.len() {
            out.push(char);
            out.push(chars[index + 1]);
            index += 2;
            continue;
        }

        if quote == Some('\'') {
            out.push(char);
            if char == '\'' {
                quote = None;
            }
            index += 1;
            continue;
        }

        if quote.is_none() && (char == '\'' || char == '"') {
            quote = Some(char);
            out.push(char);
            index += 1;
            continue;
        }

        if quote == Some('"') && char == '"' {
            quote = None;
            out.push(char);
            index += 1;
            continue;
        }

        if char == '$' && index + 1 < chars.len() && chars[index + 1] == '(' {
            let mut depth = 0;
            let mut scan = index + 1;
            while scan < chars.len() {
                if chars[scan] == '(' {
                    depth += 1;
                } else if chars[scan] == ')' {
                    depth -= 1;
                    if depth == 0 {
                        break;
                    }
                }
                scan += 1;
            }
            if scan >= chars.len() {
                out.push(char);
                index += 1;
                continue;
            }
            inners.push(chars[index + 2..scan].iter().collect());
            out.push_str(SUBST_PLACEHOLDER);
            index = scan + 1;
            continue;
        }

        if char == '`' {
            let close = chars[index + 1..].iter().position(|&c| c == '`');
            let Some(close) = close.map(|offset| index + 1 + offset) else {
                out.push(char);
                index += 1;
                continue;
            };
            inners.push(chars[index + 1..close].iter().collect());
            out.push_str(SUBST_PLACEHOLDER);
            index = close + 1;
            continue;
        }

        out.push(char);
        index += 1;
    }

    (out, inners)
}

/// Split the text into words the way the shell would, keeping the written form
/// alongside. A newline separates commands, so it stays a token of its own
/// instead of being swallowed as whitespace.
pub fn tokenize(text: &str) -> Result<Vec<Token>, Unanalyzable> {
    let chars: Vec<char> = text.chars().collect();
    let mut tokens = Vec::new();
    let mut index = 0;

    while index < chars.len() {
        let char = chars[index];

        if char == ' ' || char == '\t' || char == '\r' {
            index += 1;
            continue;
        }

        if PUNCTUATION.contains(&char) {
            let start = index;
            while index < chars.len() && PUNCTUATION.contains(&chars[index]) {
                index += 1;
            }
            let run: String = chars[start..index].iter().collect();
            tokens.push(Token {
                value: run.clone(),
                raw: run,
                separator: true,
            });
            continue;
        }

        let mut value = String::new();
        let mut raw = String::new();
        while index < chars.len() {
            let char = chars[index];
            if char == ' ' || char == '\t' || char == '\r' || PUNCTUATION.contains(&char) {
                break;
            }

            if char == '\'' {
                raw.push(char);
                index += 1;
                loop {
                    let Some(&inner) = chars.get(index) else {
                        return Err(Unanalyzable("No closing quotation".to_string()));
                    };
                    raw.push(inner);
                    index += 1;
                    if inner == '\'' {
                        break;
                    }
                    value.push(inner);
                }
                continue;
            }

            if char == '"' {
                raw.push(char);
                index += 1;
                loop {
                    let Some(&inner) = chars.get(index) else {
                        return Err(Unanalyzable("No closing quotation".to_string()));
                    };
                    if inner == '"' {
                        raw.push(inner);
                        index += 1;
                        break;
                    }
                    // Inside double quotes a backslash only escapes a quote or
                    // another backslash; anywhere else it stays part of the word.
                    if inner == '\\'
                        && let Some(&escaped) = chars.get(index + 1)
                    {
                        raw.push(inner);
                        raw.push(escaped);
                        if escaped != '"' && escaped != '\\' {
                            value.push(inner);
                        }
                        value.push(escaped);
                        index += 2;
                        continue;
                    }
                    raw.push(inner);
                    value.push(inner);
                    index += 1;
                }
                continue;
            }

            if char == '\\' {
                let Some(&escaped) = chars.get(index + 1) else {
                    return Err(Unanalyzable("No escaped character".to_string()));
                };
                raw.push(char);
                raw.push(escaped);
                value.push(escaped);
                index += 2;
                continue;
            }

            value.push(char);
            raw.push(char);
            index += 1;
        }

        tokens.push(Token {
            value,
            raw,
            separator: false,
        });
    }

    Ok(tokens)
}

/// True if the written form can expand into extra shell words (or run code).
pub fn splits_into_words(raw: &str) -> bool {
    let mut quote: Option<char> = None;
    for char in raw.chars() {
        match quote {
            None if char == '\'' || char == '"' => quote = Some(char),
            None if char == '$' || char == '`' => return true,
            Some(open) if char == open => quote = None,
            _ => {}
        }
    }
    false
}

pub fn has_expansion(raw: &str) -> bool {
    raw.contains('$') || raw.contains('`')
}

pub fn segments(tokens: &[Token]) -> Vec<&[Token]> {
    tokens
        .split(|token| token.separator)
        .filter(|segment| !segment.is_empty())
        .collect()
}

pub fn basename(path: &str) -> &str {
    path.rsplit('/').next().unwrap_or(path)
}

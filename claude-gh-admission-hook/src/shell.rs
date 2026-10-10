use regex::Regex;
use std::sync::LazyLock;

pub const SUBST_PLACEHOLDER: &str = "__SUBST__";

const BLANKS: [char; 3] = [' ', '\t', '\r'];
const SEPARATORS: [char; 6] = ['(', ')', ';', '|', '&', '\n'];
const REDIRECTORS: [char; 2] = ['<', '>'];
const REDIRECTION: [char; 5] = ['<', '>', '&', '|', '!'];
const GLOBS: [char; 3] = ['*', '?', '['];

static HEREDOC: LazyLock<Regex> = LazyLock::new(|| {
    Regex::new(r#"<<-?\s*(?:'([^']*)'|"([^"]*)"|\\?([A-Za-z_][A-Za-z0-9_]*))"#).unwrap()
});

#[derive(Clone)]
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

        let substitutes = char == '$' || (quote.is_none() && REDIRECTORS.contains(&char));
        if substitutes && index + 1 < chars.len() && chars[index + 1] == '(' {
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

pub fn tokenize(text: &str, comments: bool) -> Result<Vec<Token>, Unanalyzable> {
    let chars: Vec<char> = text.chars().collect();
    let mut tokens = Vec::new();
    let mut index = 0;

    while index < chars.len() {
        let char = chars[index];

        if BLANKS.contains(&char) {
            index += 1;
            continue;
        }

        if comments && char == '#' {
            while index < chars.len() && chars[index] != '\n' {
                index += 1;
            }
            continue;
        }

        if REDIRECTORS.contains(&char) || joins_outputs(&chars, index) {
            while index < chars.len() && REDIRECTION.contains(&chars[index]) {
                index += 1;
            }
            while index < chars.len() && BLANKS.contains(&chars[index]) {
                index += 1;
            }
            word(&chars, &mut index)?;
            continue;
        }

        if SEPARATORS.contains(&char) {
            let start = index;
            index += 1;
            while index < chars.len()
                && SEPARATORS.contains(&chars[index])
                && (chars[index - 1] == '&' || !joins_outputs(&chars, index))
            {
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

        let (value, raw) = word(&chars, &mut index)?;
        let descriptor = chars
            .get(index)
            .is_some_and(|next| REDIRECTORS.contains(next))
            && !raw.is_empty()
            && raw.chars().all(|char| char.is_ascii_digit());
        if !descriptor {
            tokens.push(Token {
                value,
                raw,
                separator: false,
            });
        }
    }

    Ok(tokens)
}

fn joins_outputs(chars: &[char], index: usize) -> bool {
    chars.get(index) == Some(&'&') && chars.get(index + 1) == Some(&'>')
}

fn word(chars: &[char], index: &mut usize) -> Result<(String, String), Unanalyzable> {
    let mut value = String::new();
    let mut raw = String::new();

    while let Some(&char) = chars.get(*index) {
        if BLANKS.contains(&char) || SEPARATORS.contains(&char) || REDIRECTORS.contains(&char) {
            break;
        }

        if char == '\'' {
            raw.push(char);
            *index += 1;
            loop {
                let Some(&inner) = chars.get(*index) else {
                    return Err(Unanalyzable("No closing quotation".to_string()));
                };
                raw.push(inner);
                *index += 1;
                if inner == '\'' {
                    break;
                }
                value.push(inner);
            }
            continue;
        }

        if char == '"' {
            raw.push(char);
            *index += 1;
            loop {
                let Some(&inner) = chars.get(*index) else {
                    return Err(Unanalyzable("No closing quotation".to_string()));
                };
                if inner == '"' {
                    raw.push(inner);
                    *index += 1;
                    break;
                }
                if inner == '\\'
                    && let Some(&escaped) = chars.get(*index + 1)
                {
                    raw.push(inner);
                    raw.push(escaped);
                    if escaped != '"' && escaped != '\\' {
                        value.push(inner);
                    }
                    value.push(escaped);
                    *index += 2;
                    continue;
                }
                raw.push(inner);
                value.push(inner);
                *index += 1;
            }
            continue;
        }

        if char == '\\' {
            let Some(&escaped) = chars.get(*index + 1) else {
                return Err(Unanalyzable("No escaped character".to_string()));
            };
            raw.push(char);
            raw.push(escaped);
            value.push(escaped);
            *index += 2;
            continue;
        }

        value.push(char);
        raw.push(char);
        *index += 1;
    }

    Ok((value, raw))
}

pub fn has_expansion(raw: &str) -> bool {
    raw.contains('$') || raw.contains('`')
}

pub fn expands(text: &str) -> bool {
    has_expansion(text) || text.contains(SUBST_PLACEHOLDER)
}

pub fn splits(token: &Token) -> bool {
    matches!(shape(&token.raw), Shape::Splitting)
}

pub fn may_be_flag(token: &Token) -> bool {
    if literal(token) {
        return false;
    }
    let value = token.value.as_str();
    if value.starts_with('-') {
        return expands(value.split('=').next().unwrap_or(value));
    }
    value.starts_with(['$', '`']) || value.starts_with(SUBST_PLACEHOLDER)
}

pub fn literal(token: &Token) -> bool {
    matches!(shape(&token.raw), Shape::Literal)
}

pub fn unseen() -> Token {
    Token {
        value: SUBST_PLACEHOLDER.to_string(),
        raw: SUBST_PLACEHOLDER.to_string(),
        separator: false,
    }
}

pub enum Shape {
    Literal,
    Opaque,
    Splitting,
}

pub fn shape(raw: &str) -> Shape {
    let mut quote: Option<char> = None;
    let mut escaped = false;
    let mut opaque = false;

    for (offset, char) in raw.char_indices() {
        if escaped {
            escaped = false;
            continue;
        }
        if quote == Some('\'') {
            if char == '\'' {
                quote = None;
            }
            continue;
        }
        if char == '\\' {
            escaped = true;
            continue;
        }

        let expands = char == '$' || char == '`' || raw[offset..].starts_with(SUBST_PLACEHOLDER);
        match quote {
            Some(_) if char == '"' => quote = None,
            Some(_) => opaque |= expands,
            None if char == '\'' || char == '"' => quote = Some(char),
            None if expands || GLOBS.contains(&char) || braces(&raw[offset..]) => {
                return Shape::Splitting;
            }
            None => {}
        }
    }

    if opaque {
        Shape::Opaque
    } else {
        Shape::Literal
    }
}

fn braces(rest: &str) -> bool {
    rest.starts_with('{') && (rest.contains(',') || rest.contains(".."))
}

pub fn shown(token: &Token) -> String {
    token.raw.replace(SUBST_PLACEHOLDER, "$(...)")
}

pub fn segments(tokens: &[Token]) -> Vec<&[Token]> {
    let mut found = Vec::new();
    let mut start = 0;

    for (index, token) in tokens.iter().enumerate() {
        if !token.separator {
            continue;
        }
        if index > start {
            found.push(&tokens[start..index]);
        }
        start = index + 1;
    }
    if start < tokens.len() {
        found.push(&tokens[start..]);
    }

    found
}

pub fn basename(path: &str) -> &str {
    path.rsplit('/').next().unwrap_or(path)
}

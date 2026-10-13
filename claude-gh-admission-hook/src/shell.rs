use regex::Regex;
use std::sync::LazyLock;

pub const SUBST_PLACEHOLDER: &str = "__SUBST__";

const BLANKS: [char; 3] = [' ', '\t', '\r'];
const SEPARATORS: [char; 6] = ['(', ')', ';', '|', '&', '\n'];
const REDIRECTORS: [char; 2] = ['<', '>'];
const REDIRECTION: [char; 5] = ['<', '>', '&', '|', '!'];
const GLOBS: [char; 3] = ['*', '?', '['];

static HEREDOC: LazyLock<Regex> = LazyLock::new(|| {
    Regex::new(r#"^<<-?[ \t]*(?:'([^']*)'|"([^"]*)"|\\?([A-Za-z_][A-Za-z0-9_]*))"#).unwrap()
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

pub struct Heredoc {
    pub body: String,
    pub expands: bool,
}

#[derive(Default)]
struct Level {
    quote: Option<char>,
    parens: u32,
    backticks: bool,
}

fn closing(chars: &[char], from: usize, pair: [char; 2]) -> Option<usize> {
    (from..chars.len().saturating_sub(1))
        .take_while(|&at| chars[at] != '\n')
        .find(|&at| chars[at] == pair[0] && chars[at + 1] == pair[1])
        .map(|at| at + 2)
}

fn body(chars: &[char], from: usize, delimiter: &str) -> Option<(String, usize)> {
    let mut lines: Vec<String> = Vec::new();
    let mut start = from;
    while start < chars.len() {
        let end = (start..chars.len())
            .find(|&at| chars[at] == '\n')
            .unwrap_or(chars.len());
        let line: String = chars[start..end].iter().collect();
        if line.trim() == delimiter {
            return Some((lines.join("\n"), (end + 1).min(chars.len())));
        }
        lines.push(line);
        start = end + 1;
    }
    None
}

pub fn strip_heredocs(text: &str) -> (String, Vec<Heredoc>) {
    let chars: Vec<char> = text.chars().collect();
    let mut out = String::new();
    let mut bodies: Vec<Heredoc> = Vec::new();
    let mut pending: Vec<(String, bool)> = Vec::new();
    let mut levels = vec![Level::default()];
    let mut index = 0;

    while index < chars.len() {
        let char = chars[index];
        let next = chars.get(index + 1).copied();
        let nested = levels.len() > 1;
        let Some(level) = levels.last_mut() else {
            break;
        };

        if level.quote == Some('\'') {
            if char == '\'' {
                level.quote = None;
            }
            out.push(char);
            index += 1;
            continue;
        }

        if char == '\\' {
            out.push(char);
            out.extend(next);
            index += 2;
            continue;
        }

        if char == '$' && next == Some('(') {
            if chars.get(index + 2) == Some(&'(')
                && let Some(end) = closing(&chars, index + 3, [')', ')'])
            {
                out.extend(&chars[index..end]);
                index = end;
                continue;
            }
            out.push_str("$(");
            levels.push(Level::default());
            index += 2;
            continue;
        }

        if char == '`' {
            if level.backticks {
                levels.pop();
            } else {
                levels.push(Level {
                    backticks: true,
                    ..Level::default()
                });
            }
            out.push(char);
            index += 1;
            continue;
        }

        if level.quote == Some('"') {
            if char == '"' {
                level.quote = None;
            }
            out.push(char);
            index += 1;
            continue;
        }

        let starts_word = out
            .chars()
            .last()
            .is_none_or(|last| last.is_whitespace() || SEPARATORS.contains(&last));

        match char {
            '\'' | '"' => level.quote = Some(char),
            '(' if next == Some('(') && starts_word => {
                if let Some(end) = closing(&chars, index + 2, [')', ')']) {
                    out.extend(&chars[index..end]);
                    index = end;
                    continue;
                }
            }
            '(' if nested => level.parens += 1,
            ')' if nested && !level.backticks => {
                if level.parens == 0 {
                    levels.pop();
                } else {
                    level.parens -= 1;
                }
            }
            '#' if starts_word => {
                while index < chars.len() && chars[index] != '\n' {
                    out.push(chars[index]);
                    index += 1;
                }
                continue;
            }
            '<' if next == Some('<') => {
                if chars.get(index + 2) == Some(&'<') {
                    out.push_str("<<<");
                    index += 3;
                    continue;
                }
                let line: String = chars[index..]
                    .iter()
                    .take_while(|char| **char != '\n')
                    .collect();
                if let Some(captures) = HEREDOC.captures(&line) {
                    let quoted = captures.get(1).or_else(|| captures.get(2));
                    let delimiter = quoted.or_else(|| captures.get(3));
                    if let Some(delimiter) = delimiter.filter(|found| !found.as_str().is_empty()) {
                        pending.push((delimiter.as_str().to_string(), quoted.is_none()));
                    }
                    let operator = &captures[0];
                    out.push_str(operator);
                    index += operator.chars().count();
                    continue;
                }
            }
            '\n' if !pending.is_empty() => {
                out.push(char);
                index += 1;
                for (delimiter, expands) in pending.drain(..) {
                    let Some((body, end)) = body(&chars, index, &delimiter) else {
                        break;
                    };
                    bodies.push(Heredoc { body, expands });
                    index = end;
                }
                continue;
            }
            _ => {}
        }

        out.push(char);
        index += 1;
    }

    (out, bodies)
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

pub type Words = (Vec<Token>, Vec<String>);

pub fn tokenize(text: &str, comments: bool) -> Result<Words, Unanalyzable> {
    let chars: Vec<char> = text.chars().collect();
    let mut tokens = Vec::new();
    let mut targets = Vec::new();
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
            targets.push(word(&chars, &mut index)?.0);
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

    Ok((tokens, targets))
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

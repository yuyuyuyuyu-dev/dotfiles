mod analyze;
mod gh;
mod git;
mod http;
mod shell;

use std::io::Read;

pub enum Verdict {
    Deny(String),
    Ask(String),
}

const DENY_NOTE: &str = " GitHub access is read-only under this hook: every write is denied, and the only exceptions are creating and editing a pull request, which are put to the user for approval instead. This is a permanent PreToolUse hook, not a transient failure: retrying, rewording or wrapping the command will not change the answer. Ask the user to run it themselves if the write is genuinely needed. If the command only reads and this denial looks like a fault in the hook, report that to the user instead of working around it.";

fn main() {
    let mut payload = String::new();
    if std::io::stdin().read_to_string(&mut payload).is_err() {
        return;
    }

    let Ok(parsed) = serde_json::from_str::<serde_json::Value>(&payload) else {
        return;
    };

    let command = parsed
        .get("tool_input")
        .and_then(|input| input.get("command"))
        .and_then(|command| command.as_str())
        .unwrap_or_default();
    if command.trim().is_empty() {
        return;
    }

    let Some(verdict) = analyze::check_command(command, 0) else {
        return;
    };

    let (decision, reason) = match verdict {
        Verdict::Deny(reason) => ("deny", reason + DENY_NOTE),
        Verdict::Ask(reason) => ("ask", reason),
    };

    print!(
        "{}",
        serde_json::json!({
            "hookSpecificOutput": {
                "hookEventName": "PreToolUse",
                "permissionDecision": decision,
                "permissionDecisionReason": reason,
            }
        })
    );
}

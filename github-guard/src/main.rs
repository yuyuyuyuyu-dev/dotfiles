//! PreToolUse(Bash) hook: GitHub is read-only from here.
//!
//! Allow-list, not deny-list, throughout: a gh invocation passes only when its
//! subcommand is named as a read, and `gh api` or curl/wget against the GitHub API
//! pass only when the effective HTTP method is provably a read. A deny-list goes
//! stale every time GitHub ships a new subcommand or endpoint, and the stale
//! direction is the one that lets writes through.
//!
//! Creating and editing a pull request are the two exceptions. They write, but
//! they are the writes this hook exists to permit, so they ask instead of denying.
//!
//! git is checked too, but only for tags: creating one locally, and the push
//! and update-ref forms that make a remote tag without a local one. Branch
//! pushes are left alone.
//!
//! The analyzer unwraps shells, wrappers and runners before matching, because a
//! permission rule matches the literal command string and so never sees the gh
//! call inside `bash -c '...'`.

mod analyze;
mod gh;
mod git;
mod http;
mod shell;

use std::io::Read;

/// What the hook decided, and why. The reason is written for the agent that is
/// about to be stopped: it has to say what was read out of the command, what
/// about it is a write, and what to do instead.
pub enum Verdict {
    Deny(String),
    Ask(String),
}

// Appended to every denial. Without it a denial reads like a transient failure,
// and the next thing the agent does is try a way around it.
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

mod analyze;
mod exception;
mod gh;
mod git;
mod github;
mod http;
mod lookup;
mod pull_request;
mod release;
mod shell;

use std::io::Read;

pub enum Verdict {
    Deny(String),
    Ask(String),
    AskIfDraftPullRequest(pull_request::PullRequest),
    AskIfDraftRelease(release::Release),
    PassIfPushLeavesTags(git::Push),
    Unsettling(&'static str),
}

const DENY_NOTE: &str = " GitHub access is read-only under this hook: every write is denied, and the only exceptions are creating a draft pull request, editing a draft pull request, creating a draft release and editing a draft release, which are put to the user for approval instead. This is a permanent PreToolUse hook, not a transient failure: retrying the command, or rewording or wrapping it to do the same thing, will not change the answer. Ask the user to run it themselves if the write is genuinely needed. If the command only reads and this denial looks like a fault in the hook, report that to the user instead of working around it.";

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

    let directory = parsed.get("cwd").and_then(|directory| directory.as_str());

    let Some((decision, reason)) = decide(analyze::check_command(command, 0), directory) else {
        return;
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

fn decide(verdicts: Vec<Verdict>, directory: Option<&str>) -> Option<(&'static str, String)> {
    let mut unsettling = None;
    let mut pushes = Vec::new();
    let mut approvals = Vec::new();
    for verdict in verdicts {
        match verdict {
            Verdict::Deny(reason) => return Some(("deny", reason + DENY_NOTE)),
            Verdict::Unsettling(what) => unsettling = unsettling.or(Some(what)),
            Verdict::PassIfPushLeavesTags(push) => pushes.push(push),
            approval => approvals.push(approval),
        }
    }

    if let Some(what) = unsettling
        && !pushes.is_empty()
    {
        return Some((
            "deny",
            format!(
                "`git push` is let through only after this hook has looked at the git configuration it would run under, and this command also {what}, so that configuration could be another one by the time the push runs. Run `git push` in a call of its own."
            ) + DENY_NOTE,
        ));
    }
    if let Some(Verdict::Deny(reason)) = pushes.iter().find_map(|push| git::judge(push, directory))
    {
        return Some(("deny", reason + DENY_NOTE));
    }

    let mut reasons: Vec<String> = Vec::new();
    for approval in approvals {
        let reason = match approval {
            Verdict::AskIfDraftPullRequest(pull_request) => pull_request::judge(&pull_request),
            Verdict::AskIfDraftRelease(release) => release::judge(&release),
            settled => settled,
        };
        match reason {
            Verdict::Deny(reason) => return Some(("deny", reason + DENY_NOTE)),
            Verdict::Ask(reason) if !reasons.contains(&reason) => reasons.push(reason),
            _ => {}
        }
    }

    if reasons.is_empty() {
        return None;
    }
    Some(("ask", reasons.join(" ")))
}

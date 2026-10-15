use crate::Verdict;

const PULL_REQUEST_TOOLS: &str = "mcp__ccd_pr__";

const PULL_REQUEST_TOOLS_THAT_LEAVE_GITHUB_ALONE: [&str; 4] = [
    "mcp__ccd_pr__bind_pr",
    "mcp__ccd_pr__get_status",
    "mcp__ccd_pr__set_monitor",
    "mcp__ccd_pr__unbind_pr",
];

pub fn check(name: &str) -> Option<Verdict> {
    if !name.starts_with(PULL_REQUEST_TOOLS)
        || PULL_REQUEST_TOOLS_THAT_LEAVE_GITHUB_ALONE.contains(&name)
    {
        return None;
    }
    Some(Verdict::Deny(format!(
        "`{name}` is not on this hook's allow-list of the pull request tools of the Claude desktop app that leave GitHub unchanged, so it cannot be shown to leave GitHub unchanged. If `{name}` really leaves GitHub unchanged, the allow-list in claude-gh-admission-hook is what needs the entry."
    )))
}

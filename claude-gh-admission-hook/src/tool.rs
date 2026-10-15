use crate::Verdict;

const AUTO_MERGE: &str = "mcp__ccd_pr__set_auto_merge";

pub fn check(name: &str) -> Option<Verdict> {
    if name != AUTO_MERGE {
        return None;
    }
    Some(Verdict::Deny(format!(
        "`{AUTO_MERGE}` turns auto-merge on or off for a pull request on GitHub, and either way that changes GitHub state."
    )))
}

use crate::Verdict;
use crate::github::{self, Repository};

const STATE_AND_DRAFT: &str = r#""\(.state) \(.isDraft)""#;

pub struct PullRequest {
    pub repository: Repository,
    pub number: String,
}

pub fn judge(pull_request: &PullRequest) -> Verdict {
    let PullRequest { repository, number } = pull_request;

    let found = github::ask(&[
        "pr",
        "view",
        "--repo",
        &repository.to_string(),
        "--json",
        "isDraft,state",
        "--jq",
        STATE_AND_DRAFT,
        "--",
        number,
    ])
    .and_then(|answer| match answer.stdout.trim() {
        "OPEN true" => Ok(None),
        "OPEN false" => Ok(Some("ready for review")),
        "CLOSED true" | "CLOSED false" => Ok(Some("closed")),
        "MERGED true" | "MERGED false" => Ok(Some("merged")),
        _ => Err(answer.failure()),
    });

    match found {
        Ok(None) => Verdict::Ask(format!(
            "Pull request {number} in {repository} is an open draft. Editing a draft pull request is one of the few writes this hook allows, so it needs the user to approve it rather than being denied."
        )),
        Ok(Some(state)) => Verdict::Deny(format!(
            "Pull request {number} in {repository} is {state}, and only a draft pull request that is still open may be edited. Changing this one is for the user to do."
        )),
        Err(failure) => Verdict::Deny(format!(
            "`gh pr edit` is only allowed for a draft pull request, and whether {number} in {repository} is one could not be found out: `gh pr view` {failure}. Check the number and the repository, and that gh is signed in to {}.",
            repository.host
        )),
    }
}

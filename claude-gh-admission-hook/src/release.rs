use crate::Verdict;
use crate::github::{self, Repository};

pub struct Release {
    pub repository: Repository,
    pub tag: String,
}

pub fn judge(release: &Release) -> Verdict {
    let Release { repository, tag } = release;

    match is_draft(release) {
        Ok(true) => {}
        Ok(false) => {
            return Verdict::Deny(format!(
                "Release {tag} in {repository} is published, and only a draft release may be edited. Changing a published release is for the user to do."
            ));
        }
        Err(failure) => {
            return Verdict::Deny(format!(
                "`gh release edit` is only allowed for a draft release, and whether {tag} in {repository} is one could not be found out: `gh release view` {failure}. Check the tag and the repository, and that gh is signed in to {}.",
                repository.host
            ));
        }
    }

    match has_published_twin(release) {
        Ok(false) => Verdict::Ask(format!(
            "Release {tag} in {repository} is a draft, and no published release shares its tag. Editing a draft release is one of the few writes this hook allows, so it needs the user to approve it rather than being denied."
        )),
        Ok(true) => Verdict::Deny(format!(
            "{repository} holds a draft release with the tag {tag} and a published release with the same tag, and `gh release edit` could pick either of them. As long as both exist, editing the draft is for the user to do."
        )),
        Err(failure) => Verdict::Deny(format!(
            "Release {tag} in {repository} is a draft, but whether a published release shares its tag could not be found out: `gh api` {failure}. If one does, `gh release edit` could pick it instead, so the edit is not let through."
        )),
    }
}

fn is_draft(release: &Release) -> Result<bool, String> {
    let answer = github::ask(&[
        "release",
        "view",
        "--repo",
        &release.repository.to_string(),
        "--json",
        "isDraft",
        "--jq",
        ".isDraft",
        "--",
        &release.tag,
    ])?;
    match answer.stdout.trim() {
        "true" => Ok(true),
        "false" => Ok(false),
        _ => Err(answer.failure()),
    }
}

fn has_published_twin(release: &Release) -> Result<bool, String> {
    let endpoint = format!(
        "repos/{}/{}/releases/tags/{}",
        release.repository.owner,
        release.repository.name,
        release.tag.replace('/', "%2F")
    );
    let answer = github::ask(&[
        "api",
        "--hostname",
        &release.repository.host,
        "--include",
        "--silent",
        &endpoint,
    ])?;
    match answer.stdout.split_whitespace().nth(1) {
        Some("404") => Ok(false),
        Some("200") => Ok(true),
        _ => Err(answer.failure()),
    }
}

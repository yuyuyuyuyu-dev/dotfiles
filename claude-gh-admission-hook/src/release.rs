use crate::Verdict;
use std::io::Read;
use std::process::{Command, Stdio};
use std::sync::mpsc::{self, Receiver};
use std::thread;
use std::time::{Duration, Instant};

const PATIENCE: Duration = Duration::from_secs(10);

pub struct Release {
    pub host: String,
    pub owner: String,
    pub name: String,
    pub tag: String,
}

struct Answer {
    stdout: String,
    stderr: String,
}

pub fn judge(release: &Release) -> Verdict {
    let Release {
        host,
        owner,
        name,
        tag,
    } = release;
    let repository = format!("{host}/{owner}/{name}");

    match is_draft(&repository, tag) {
        Ok(true) => {}
        Ok(false) => {
            return Verdict::Deny(format!(
                "Release {tag} in {repository} is published, and only a draft release may be edited. Changing a published release is for the user to do."
            ));
        }
        Err(failure) => {
            return Verdict::Deny(format!(
                "`gh release edit` is only allowed for a draft release, and whether {tag} in {repository} is one could not be found out: `gh release view` {failure}. Check the tag and the repository, and that gh is signed in to {host}."
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

fn is_draft(repository: &str, tag: &str) -> Result<bool, String> {
    let answer = gh(&[
        "release", "view", "--repo", repository, "--json", "isDraft", "--jq", ".isDraft", "--", tag,
    ])?;
    match answer.stdout.trim() {
        "true" => Ok(true),
        "false" => Ok(false),
        _ => Err(failed(&answer)),
    }
}

fn has_published_twin(release: &Release) -> Result<bool, String> {
    let endpoint = format!(
        "repos/{}/{}/releases/tags/{}",
        release.owner,
        release.name,
        release.tag.replace('/', "%2F")
    );
    let answer = gh(&[
        "api",
        "--hostname",
        &release.host,
        "--include",
        "--silent",
        &endpoint,
    ])?;
    match answer.stdout.split_whitespace().nth(1) {
        Some("404") => Ok(false),
        Some("200") => Ok(true),
        _ => Err(failed(&answer)),
    }
}

fn failed(answer: &Answer) -> String {
    let said = answer
        .stderr
        .lines()
        .map(str::trim)
        .find(|line| !line.is_empty());
    match said {
        Some(line) => format!("failed ({})", line.chars().take(200).collect::<String>()),
        None => "failed without saying why".to_string(),
    }
}

fn gh(arguments: &[&str]) -> Result<Answer, String> {
    let mut child = Command::new("gh")
        .args(arguments)
        .env_remove("GH_FORCE_TTY")
        .env_remove("CLICOLOR_FORCE")
        .stdin(Stdio::null())
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .spawn()
        .map_err(|error| format!("could not be started ({error})"))?;
    let deadline = Instant::now() + PATIENCE;
    let stdout = collect(child.stdout.take());
    let stderr = collect(child.stderr.take());

    let answer = wait(&stdout, deadline)
        .and_then(|stdout| wait(&stderr, deadline).map(|stderr| Answer { stdout, stderr }));
    let _ = child.kill();
    let _ = child.wait();

    answer.ok_or_else(|| format!("did not answer within {} seconds", PATIENCE.as_secs()))
}

fn collect<R: Read + Send + 'static>(pipe: Option<R>) -> Receiver<String> {
    let (sender, receiver) = mpsc::channel();
    let _ = thread::Builder::new().spawn(move || {
        let mut text = String::new();
        if let Some(mut pipe) = pipe {
            let _ = pipe.read_to_string(&mut text);
        }
        let _ = sender.send(text);
    });
    receiver
}

fn wait(pipe: &Receiver<String>, deadline: Instant) -> Option<String> {
    pipe.recv_timeout(deadline.saturating_duration_since(Instant::now()))
        .ok()
}

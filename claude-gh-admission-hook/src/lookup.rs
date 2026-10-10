use std::io::Read;
use std::process::{Command, Stdio};
use std::sync::mpsc::{self, Receiver};
use std::thread;
use std::time::{Duration, Instant};

const PATIENCE: Duration = Duration::from_secs(10);

pub struct Answer {
    pub stdout: String,
    stderr: String,
    pub status: Option<i32>,
}

impl Answer {
    pub fn failure(&self) -> String {
        let said = self
            .stderr
            .lines()
            .map(str::trim)
            .find(|line| !line.is_empty());
        match said {
            Some(line) => format!("failed ({})", line.chars().take(200).collect::<String>()),
            None => "failed without saying why".to_string(),
        }
    }
}

pub fn run(mut command: Command) -> Result<Answer, String> {
    let mut child = command
        .stdin(Stdio::null())
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .spawn()
        .map_err(|error| format!("could not be started ({error})"))?;
    let deadline = Instant::now() + PATIENCE;
    let stdout = collect(child.stdout.take());
    let stderr = collect(child.stderr.take());

    let texts = wait(&stdout, deadline)
        .and_then(|stdout| wait(&stderr, deadline).map(|stderr| (stdout, stderr)));
    let _ = child.kill();
    let status = child.wait().ok().and_then(|status| status.code());

    texts
        .map(|(stdout, stderr)| Answer {
            stdout,
            stderr,
            status,
        })
        .ok_or_else(|| format!("did not answer within {} seconds", PATIENCE.as_secs()))
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

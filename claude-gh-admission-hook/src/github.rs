use crate::lookup::{self, Answer};
use std::fmt;
use std::process::Command;

pub struct Repository {
    pub host: String,
    pub owner: String,
    pub name: String,
}

impl fmt::Display for Repository {
    fn fmt(&self, out: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(out, "{}/{}/{}", self.host, self.owner, self.name)
    }
}

pub fn ask(arguments: &[&str]) -> Result<Answer, String> {
    let mut command = Command::new("gh");
    command
        .args(arguments)
        .env_remove("GH_FORCE_TTY")
        .env_remove("CLICOLOR_FORCE");
    lookup::run(command)
}

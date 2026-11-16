//! Error when command exited with non success exit status.
use crate::types::{command::Command, output::Output};
use core::{
    error::Error,
    fmt::{Display, Formatter, Result},
};
use std::process::ExitStatus;

/// Error when command exited with non success exit status.
#[derive(Debug)]
pub struct RunCommandError {
    /// Command that failed.
    command: Command,
    /// Exit status of the failed command.
    exit_status: ExitStatus,
    /// Output for the failed command.
    output: Output,
}

impl Display for RunCommandError {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        write!(
            f,
            "Command {} failed with status {} and error: {}",
            self.command,
            match self.exit_status.code() {
                Some(code) => code.to_string(),
                None => "None".to_owned(),
            },
            self.output.err,
        )
    }
}

impl Error for RunCommandError {}

impl RunCommandError {
    /// Create new runtime error for the command with exit status and output.
    pub(crate) fn new(command: Command, exit_status: ExitStatus, output: Output) -> Self {
        Self {
            command,
            exit_status,
            output,
        }
    }
}

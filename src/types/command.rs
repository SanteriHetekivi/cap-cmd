//! Command that was ran.

use core::{
    fmt::{Display, Formatter, Result},
    iter::once,
};
use std::ffi::{OsStr, OsString};

/// Command that was ran.
#[derive(Debug)]
pub(crate) struct Command {
    /// Arguments for the program.
    args: Vec<OsString>,
    /// Program that the command runs.
    program: OsString,
}

impl Display for Command {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        write!(
            f,
            "{}",
            once(self.program.to_string_lossy())
                .chain(self.args.iter().map(|arg| arg.to_string_lossy()))
                .collect::<Vec<_>>()
                .join(" ")
        )
    }
}

impl Command {
    /// Create new command from program and arguments.
    pub(crate) fn new(
        program: impl AsRef<OsStr>,
        args: impl IntoIterator<Item = impl AsRef<OsStr>>,
    ) -> Self {
        Self {
            program: program.as_ref().to_os_string(),
            args: args
                .into_iter()
                .map(|arg| arg.as_ref().to_os_string())
                .collect(),
        }
    }
}

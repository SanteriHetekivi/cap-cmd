//! Error from cmd-function.

use super::run_command_error::RunCommandError;
use crate::types::stream::Stream;
use core::{
    error::Error,
    fmt::{Display, Formatter, Result},
};
use std::io::Error as IoError;

/// Error from cmd-function.
#[derive(Debug)]
pub enum CmdError {
    /// Input/Output error for the command itself.
    Io(IoError),
    /// Did not get pipe for one of the streams.
    NoPipe(Stream),
    /// Error when command exited with non success exit status.
    RunCommand(RunCommandError),
    /// Input/Output error for streams.
    StreamIo {
        /// Stream that error happened for.
        stream: Stream,
        /// Input output error itself.
        error: IoError,
    },
    /// Stream's thread paniced.
    ThreadPanic(Stream, String),
}

// Implement Display to control the user-facing message.
impl Display for CmdError {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        match self {
            Self::Io(error) => write!(f, "IO error: {error}"),
            Self::NoPipe(stream) => write!(f, "No {stream} pipe gotten"),
            Self::RunCommand(run_command_error) => {
                write!(f, "Command failed: {run_command_error}")
            }
            Self::StreamIo { stream, error } => {
                write!(f, "Stream {stream} had IO error {error}")
            }
            Self::ThreadPanic(stream, message) => {
                write!(f, "{stream} reader thread panicked with message: {message}")
            }
        }
    }
}

impl Error for CmdError {
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        match self {
            Self::Io(error) | Self::StreamIo { error, .. } => Some(error),
            Self::RunCommand(run_command_error) => Some(run_command_error),
            Self::NoPipe(_) | Self::ThreadPanic(_, _) => None,
        }
    }
}

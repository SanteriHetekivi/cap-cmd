//! Write output to given writers at real time and also stores it for error or retuning it if succeeded.
pub mod errors;
pub mod types;

use core::any::Any;
use errors::{cmd_error::CmdError, run_command_error::RunCommandError};
use std::{
    ffi::{OsStr, OsString},
    io::{self, Error as IoError, Write},
    process::{Child, ChildStderr, ChildStdout, Command as StDProcessCommand, ExitStatus, Stdio},
    thread::{self, ScopedJoinHandle},
};
use types::{command::Command, output::Output, stream::Stream, writer_with_data::WriterWithData};

/// Collect message from panic payload.
fn panic_message(payload: &(dyn Any + Send)) -> String {
    payload
        .downcast_ref::<&str>()
        .map(|str| (*str).to_owned())
        .or_else(|| payload.downcast_ref::<String>().cloned())
        .unwrap_or_else(|| "non-string panic payload".to_owned())
}

/// Run given program with given arguments.
/// Stream it's output to given writers and collect it.
///
/// # Errors
/// * [`CmdError::Io`] - There was input/output error for the command itself.
/// * [`CmdError::NoPipe`] - Did not get pipe for one of the streams.
/// * [`CmdError::RunCommand`] - Command exited with non success status.
/// * [`CmdError::StreamIo`] - There was input/output error for one of the streams.
/// * [`CmdError::ThreadPanic`] - One of the stream's thread paniced.
pub fn cmd<P, A, S, O, E>(
    program: P,
    args: A,
    stdout: &mut O,
    stderr: &mut E,
) -> Result<Output, CmdError>
where
    P: AsRef<OsStr>,
    A: IntoIterator<Item = S>,
    S: AsRef<OsStr>,
    O: Write + Send,
    E: Write + Send,
{
    let program_os_string: OsString = program.as_ref().to_os_string();
    let args_os_strings: Vec<OsString> = args
        .into_iter()
        .map(|arg| arg.as_ref().to_os_string())
        .collect();

    // Run the command.
    let mut child: Child = StDProcessCommand::new(&program_os_string)
        .args(&args_os_strings)
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .spawn()
        .map_err(CmdError::Io)?;

    // Read child's stdout and stderr and pass it to writers and collect.
    let mut child_stdout: ChildStdout = child
        .stdout
        .take()
        .ok_or(CmdError::NoPipe(Stream::Stdout))?;
    let mut child_stderr: ChildStderr = child
        .stderr
        .take()
        .ok_or(CmdError::NoPipe(Stream::Stderr))?;
    let mut out: WriterWithData<&mut O> = WriterWithData::new(stdout);
    let mut err: WriterWithData<&mut E> = WriterWithData::new(stderr);
    thread::scope(|scope| {
        let out_handle: ScopedJoinHandle<'_, Result<u64, IoError>> =
            scope.spawn(|| io::copy(&mut child_stdout, &mut out));
        let err_handle: ScopedJoinHandle<'_, Result<u64, IoError>> =
            scope.spawn(|| io::copy(&mut child_stderr, &mut err));
        let out_result: Result<Result<u64, IoError>, Box<dyn Any + Send>> = out_handle.join();
        let err_result: Result<Result<u64, IoError>, Box<dyn Any + Send>> = err_handle.join();
        let _: u64 = out_result
            .map_err(|payload| CmdError::ThreadPanic(Stream::Stdout, panic_message(&*payload)))?
            .map_err(CmdError::Io)?;
        let _: u64 = err_result
            .map_err(|payload| CmdError::ThreadPanic(Stream::Stderr, panic_message(&*payload)))?
            .map_err(CmdError::Io)?;
        Ok::<(), CmdError>(())
    })?;

    // Check if command succeeded.
    let exit_status: ExitStatus = child.wait().map_err(CmdError::Io)?;
    let output: Output = Output::new(out.as_bytes(), err.as_bytes());
    if !exit_status.success() {
        return Err(CmdError::RunCommand(RunCommandError::new(
            Command::new(program_os_string, args_os_strings),
            exit_status,
            output,
        )));
    }

    Ok(output)
}

/// Run given program with given arguments.
/// Stream it's output to standard io and collect it.
///
/// # Errors
/// * [`CmdError::Io`] - There was input/output error for the command itself.
/// * [`CmdError::NoPipe`] - Did not get pipe for one of the streams.
/// * [`CmdError::RunCommand`] - Command exited with non success status.
/// * [`CmdError::StreamIo`] - There was input/output error for one of the streams.
/// * [`CmdError::ThreadPanic`] - One of the stream's thread paniced.
pub fn cmd_tee<P, A, S>(program: P, args: A) -> Result<Output, CmdError>
where
    P: AsRef<OsStr>,
    A: IntoIterator<Item = S>,
    S: AsRef<OsStr>,
{
    cmd(program, args, &mut io::stdout(), &mut io::stderr())
}

#[cfg(test)]
mod tests;

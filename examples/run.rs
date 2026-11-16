//! Run given command, output it to stdout and stderr while running and also storing output.
use core::{error::Error, iter::Skip};
use std::{
    env::{self, ArgsOs},
    ffi::OsString,
};

use cap_cmd::{self, types::output::Output};

fn main() -> Result<(), Box<dyn Error>> {
    let mut args: Skip<ArgsOs> = env::args_os().skip(1);
    let program: OsString = args.next().ok_or("usage: run <program> [args...]")?;
    let arguments: Vec<OsString> = args.collect();

    // Output was written to standard output and error and you also have it here in _output.
    // Also if command failed, the error has it.
    let output: Output = cap_cmd::cmd_tee(&program, &arguments)?;
    // Same as:
    // let _output: Output = cmd(&program, &arguments, &mut io::stdout(), &mut io::stderr())?;

    let _stdout_raw_bytes: &[u8] = output.out.as_bytes();
    let _stdout_string: String = output.out.to_string();

    let _stderr_raw_bytes: &[u8] = output.err.as_bytes();
    let _stderr_string: String = output.err.to_string();

    Ok(())
}

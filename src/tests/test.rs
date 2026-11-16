use crate::{CmdError, Output, types::output_raw::OutputRaw};
use core::error::Error;
use std::ffi::OsStr;

/// Test strings for echo command.
#[cfg(unix)]
const ECHO_TEST_STRINGS: [&str; 2] = ["", "Hello"];

/// Check writer and raw output against expected.
#[track_caller]
fn check_output(writer: &[u8], output_raw: &OutputRaw, expected: &str) {
    let expected_string: String = expected.to_owned();
    assert_eq!(output_raw.to_string(), expected_string);
    let expected_bytes: Vec<u8> = expected_string.into_bytes();
    assert_eq!(writer, expected_bytes);
    assert_eq!(output_raw.as_bytes(), expected_bytes);
}

/// Run given command and check it's output.
#[track_caller]
fn run_cmd(
    program: impl AsRef<OsStr>,
    args: impl IntoIterator<Item = impl AsRef<OsStr>>,
    expected_stdout: &str,
    expected_stderr: &str,
) -> Result<(), CmdError> {
    let mut stdout: Vec<u8> = Vec::<u8>::new();
    let mut stderr: Vec<u8> = Vec::<u8>::new();
    let output: Output = crate::cmd(program, args, &mut stdout, &mut stderr)?;

    check_output(&stdout, &output.out, expected_stdout);
    check_output(&stderr, &output.err, expected_stderr);
    Ok(())
}

/// Check only raw output.
#[track_caller]
fn check_output_raw(output_raw: &OutputRaw, expected: &str) {
    let expected_string: String = expected.to_owned();
    assert_eq!(output_raw.to_string(), expected_string);
    assert_eq!(output_raw.as_bytes(), expected_string.into_bytes());
}

/// Run command, with writter as standard output.
#[track_caller]
fn run_cmd_tee(
    program: impl AsRef<OsStr>,
    args: impl IntoIterator<Item = impl AsRef<OsStr>>,
    expected_stdout: &str,
    expected_stderr: &str,
) -> Result<(), CmdError> {
    let output: Output = crate::cmd_tee(program, args)?;
    check_output_raw(&output.out, expected_stdout);
    check_output_raw(&output.err, expected_stderr);
    Ok(())
}

/// Run echo command.
#[cfg(unix)]
#[track_caller]
fn run_echo(string: &str) -> Result<(), CmdError> {
    run_cmd("echo", [string], &format!("{string}\n"), "")
}

/// Run echo command and output to standard output.
#[cfg(unix)]
#[track_caller]
fn run_echo_tee(string: &str) -> Result<(), CmdError> {
    run_cmd_tee("echo", [string], &format!("{string}\n"), "")
}

/// Get all UTC-8 characters.
#[track_caller]
fn all_utc8_chars() -> String {
    let all_utf8_chars: String = (0..=0x0010_FFFF_u32).filter_map(char::from_u32).collect();
    assert_eq!(all_utf8_chars.chars().count(), 1_112_064);
    all_utf8_chars
}

/// Test running echo command.
#[cfg(unix)]
#[test]
fn cmd_echo() -> Result<(), CmdError> {
    for string in ECHO_TEST_STRINGS {
        run_echo(string)?;
    }
    Ok(())
}

/// Test running cat command.
#[cfg(unix)]
#[test]
fn cmd_cat() -> Result<(), Box<dyn Error>> {
    use crate::tests::temp_file::TempFile;

    let all_utf8_chars: String = all_utc8_chars();
    run_cmd(
        "cat",
        [TempFile::new(all_utf8_chars.as_bytes())?.path_as_os_str()],
        &all_utf8_chars,
        "",
    )?;

    Ok(())
}

/// Test tee logic by running echo.
#[cfg(unix)]
#[test]
fn tee() -> Result<(), CmdError> {
    for string in ECHO_TEST_STRINGS {
        run_echo_tee(string)?;
    }
    Ok(())
}

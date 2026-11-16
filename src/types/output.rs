//! Output from the command.
use super::output_raw::OutputRaw;

/// Output from the command.
#[derive(Debug)]
pub struct Output {
    /// Standard error from command that was ran.
    pub err: OutputRaw,
    /// Standard output from command that was ran.
    pub out: OutputRaw,
}

impl Output {
    /// Create new output from raw output and error bytes.
    pub(crate) fn new(out: &[u8], err: &[u8]) -> Self {
        Self {
            out: OutputRaw::new(out),
            err: OutputRaw::new(err),
        }
    }
}

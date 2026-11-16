//! Raw output from the command.
use core::fmt::{Display, Formatter, Result};
/// Raw output from the command.
#[derive(Debug)]
pub struct OutputRaw {
    /// Raw output bytes.
    output: Vec<u8>,
}

impl OutputRaw {
    /// Get raw output bytes.
    #[must_use]
    pub fn as_bytes(&self) -> &[u8] {
        &self.output
    }

    /// Create new raw output struct from given bytes.
    pub(crate) fn new(output: &[u8]) -> Self {
        Self {
            output: output.to_vec(),
        }
    }
}

impl Display for OutputRaw {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        write!(f, "{}", String::from_utf8_lossy(&self.output))
    }
}

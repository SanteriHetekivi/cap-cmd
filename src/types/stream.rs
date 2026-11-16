//! Strams we handle.

use core::fmt::{Display, Formatter, Result};

/// Strams we handle.
#[derive(Debug, Clone, Copy)]
pub enum Stream {
    /// Standard error.
    Stderr,
    /// Standard output.
    Stdout,
}

impl Stream {
    /// Emun cases string representation.
    #[must_use]
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Stdout => "stdout",
            Self::Stderr => "stderr",
        }
    }
}

impl Display for Stream {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        f.write_str(self.as_str())
    }
}

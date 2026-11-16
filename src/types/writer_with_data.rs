//! Writer that also keeps the data it writes.
use std::io::{Result, Write};

/// Writer that also keeps the data it writes.
pub(crate) struct WriterWithData<A> {
    /// Data that writer has written.
    data: Vec<u8>,
    /// Writer that this writes to.
    writer: A,
}

impl<A: Write> Write for WriterWithData<A> {
    fn flush(&mut self) -> Result<()> {
        self.writer.flush()
    }

    fn write(&mut self, buf: &[u8]) -> Result<usize> {
        self.data.extend_from_slice(buf);
        self.writer.write_all(buf)?;
        Ok(buf.len())
    }
}

impl<A: Write> WriterWithData<A> {
    /// Get raw written bytes.
    #[must_use]
    pub fn as_bytes(&self) -> &[u8] {
        &self.data
    }

    /// Create new writer with it's own data from given writer.
    pub(crate) fn new(writer: A) -> Self {
        Self {
            writer,
            data: Vec::<u8>::new(),
        }
    }
}

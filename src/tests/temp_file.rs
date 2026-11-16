//! Generate temporary file without dependencies.
use core::sync::atomic::{AtomicUsize, Ordering};
use std::{
    env::temp_dir,
    ffi::OsStr,
    fs::{self, File, OpenOptions},
    io::{self, Write as _},
    path::PathBuf,
    process::id,
};

const CARGO_PKG_NAME: &str = env!("CARGO_PKG_NAME");

static COUNTER: AtomicUsize = AtomicUsize::new(0);

/// Struct holding temporary file that is removed when struct is dropped.
pub(super) struct TempFile {
    path: PathBuf,
}

impl TempFile {
    /// Create file with given contents.
    pub(super) fn new(contents: &[u8]) -> io::Result<Self> {
        let path: PathBuf = temp_dir().join(format!(
            "{CARGO_PKG_NAME}-test-{}-{}",
            id(),
            COUNTER.fetch_add(1, Ordering::Relaxed),
        ));
        let mut options: OpenOptions = OpenOptions::new();
        let _: &mut OpenOptions = options.write(true).create_new(true);
        #[cfg(unix)]
        {
            use std::os::unix::fs::OpenOptionsExt as _;
            let _: &mut OpenOptions = options.mode(0o600);
        }

        let mut file: File = options.open(&path)?;
        let guard: TempFile = Self { path };
        file.write_all(contents)?;
        Ok(guard)
    }

    /// Filepath as os string.
    pub(super) fn path_as_os_str(&self) -> &OsStr {
        self.path.as_os_str()
    }
}

impl Drop for TempFile {
    /// Removes the file when struct is dropped.
    fn drop(&mut self) {
        drop(fs::remove_file(&self.path));
    }
}

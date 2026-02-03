// This is free and unencumbered software released into the public domain.

use alloc::format;
use camino::Utf8PathBuf;
use derive_more::Display;
use std::{
    io::{Error, ErrorKind, Result},
    path::Path,
};

/// A program directory stored on a file system (e.g., `$HOME/.asimov/libexec/`).
#[derive(Debug, Display)]
#[display("ProgramDirectory({:?})", path)]
pub struct ProgramDirectory {
    path: Utf8PathBuf,
}

impl ProgramDirectory {
    /// Opens a program directory from a file system path.
    pub fn open(path: impl AsRef<Path>) -> Result<Self> {
        let path = path.as_ref();
        if !path.exists() {
            std::fs::create_dir_all(path)?;
        }
        let path = Utf8PathBuf::from_path_buf(path.to_path_buf()).map_err(|e| {
            Error::new(
                ErrorKind::InvalidFilename,
                format!("failed to open non-UTF-8 path: {}", e.display()),
            )
        })?;
        Ok(ProgramDirectory { path })
    }

    pub fn as_str(&self) -> &str {
        self.path.as_str()
    }
}

impl AsRef<str> for ProgramDirectory {
    fn as_ref(&self) -> &str {
        self.path.as_str()
    }
}

impl AsRef<Path> for ProgramDirectory {
    fn as_ref(&self) -> &Path {
        self.path.as_std_path()
    }
}

#[cfg(feature = "camino")]
impl AsRef<Utf8Path> for ProgramDirectory {
    fn as_ref(&self) -> &Utf8Path {
        self.path.as_path()
    }
}

impl crate::ProgramDirectory for ProgramDirectory {}

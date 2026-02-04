// This is free and unencumbered software released into the public domain.

use super::StateDirectory;
use alloc::format;
use camino::Utf8PathBuf;
use derive_more::Display;
use std::{
    io::{Error, ErrorKind, Result},
    path::Path,
};

/// A configuration directory stored on a file system (e.g., `$HOME/.asimov/configs/`).
#[derive(Debug, Display)]
#[display("ConfigDirectory({:?})", path)]
pub struct ConfigDirectory {
    path: Utf8PathBuf,
}

impl ConfigDirectory {
    /// Opens the default configuration directory in the user's home directory.
    ///
    /// On Unix platforms, including macOS and Linux, this is `$HOME/.asimov/configs/`.
    pub fn home() -> Result<Self> {
        StateDirectory::home().map(|base_dir| base_dir.configs())?
    }

    /// Opens a configuration directory from a file system path.
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
        Ok(ConfigDirectory { path })
    }

    pub fn as_str(&self) -> &str {
        self.path.as_str()
    }
}

impl AsRef<str> for ConfigDirectory {
    fn as_ref(&self) -> &str {
        self.path.as_str()
    }
}

impl AsRef<Path> for ConfigDirectory {
    fn as_ref(&self) -> &Path {
        self.path.as_std_path()
    }
}

#[cfg(feature = "camino")]
impl AsRef<Utf8Path> for ConfigDirectory {
    fn as_ref(&self) -> &Utf8Path {
        self.path.as_path()
    }
}

impl crate::ConfigDirectory for ConfigDirectory {}

// This is free and unencumbered software released into the public domain.

use alloc::{borrow::Cow, format, string::String};
use asimov_core::Named;
use camino::Utf8PathBuf;
use derive_more::Display;
use std::{
    io::{Error, ErrorKind, Result},
    path::Path,
};

/// A configuration profile stored on a file system (e.g., `$HOME/.asimov/configs/default/`).
#[derive(Debug, Display)]
#[display("ConfigProfile({:?})", path)]
pub struct ConfigProfile {
    path: Utf8PathBuf,
}

impl ConfigProfile {
    /// Opens a configuration profile from a file system path.
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
        Ok(ConfigProfile { path })
    }

    /// The name of the configuration profile (e.g., "default").
    fn name(&self) -> Cow<'_, str> {
        if let Some(name) = self.path.file_name() {
            return Cow::Borrowed(name);
        }
        self.path
            .canonicalize_utf8()
            .ok()
            .and_then(|p| p.file_name().map(String::from))
            .map_or(Cow::default(), Cow::Owned)
    }

    pub fn as_str(&self) -> &str {
        self.path.as_str()
    }
}

impl AsRef<str> for ConfigProfile {
    fn as_ref(&self) -> &str {
        self.path.as_str()
    }
}

impl AsRef<Path> for ConfigProfile {
    fn as_ref(&self) -> &Path {
        self.path.as_std_path()
    }
}

#[cfg(feature = "camino")]
impl AsRef<Utf8Path> for ConfigProfile {
    fn as_ref(&self) -> &Utf8Path {
        self.path.as_path()
    }
}

impl Named for ConfigProfile {
    fn name(&self) -> Cow<'_, str> {
        self.name()
    }
}

impl crate::ConfigProfile for ConfigProfile {
    fn prompt(&self) -> Option<String> {
        None // TODO
    }
}

// This is free and unencumbered software released into the public domain.

use super::{ConfigDirectory, ModuleDirectory, ProgramDirectory};
use alloc::format;
use camino::Utf8PathBuf;
use derive_more::Display;
use std::{
    io::{Error, ErrorKind, Result},
    path::Path,
};

/// A state directory stored on a file system (e.g., `$HOME/.asimov/`).
#[derive(Debug, Display)]
#[display("StateDirectory({:?})", path)]
pub struct StateDirectory {
    path: Utf8PathBuf,
}

impl StateDirectory {
    /// Opens a state directory from a file system path.
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
        Ok(StateDirectory { path })
    }

    /// Opens the configuration directory under this state directory.
    pub fn configs(&self) -> Result<ConfigDirectory> {
        ConfigDirectory::open(self.path.join("configs"))
    }

    /// Opens the module directory under this state directory.
    pub fn modules(&self) -> Result<ModuleDirectory> {
        ModuleDirectory::open(self.path.join("modules"))
    }

    /// Opens the program directory under this state directory.
    pub fn programs(&self) -> Result<ProgramDirectory> {
        ProgramDirectory::open(self.path.join("libexec"))
    }

    pub fn as_str(&self) -> &str {
        self.path.as_str()
    }
}

impl AsRef<str> for StateDirectory {
    fn as_ref(&self) -> &str {
        self.path.as_str()
    }
}

impl AsRef<Path> for StateDirectory {
    fn as_ref(&self) -> &Path {
        self.path.as_std_path()
    }
}

#[cfg(feature = "camino")]
impl AsRef<Utf8Path> for StateDirectory {
    fn as_ref(&self) -> &Utf8Path {
        self.path.as_path()
    }
}

impl crate::StateDirectory for StateDirectory {
    fn has_configs(&self) -> bool {
        self.path.join("configs").exists()
    }

    fn has_modules(&self) -> bool {
        self.path.join("modules").exists()
    }

    fn has_programs(&self) -> bool {
        self.path.join("libexec").exists()
    }
}

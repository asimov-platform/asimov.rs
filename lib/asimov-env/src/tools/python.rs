// This is free and unencumbered software released into the public domain.

use crate::paths::python_env;
use std::{borrow::Cow, path::PathBuf, process::Command};

pub fn python() -> Option<Cow<'static, str>> {
    getenv::python()
        .map(Cow::from)
        .or_else(|| Some(Cow::from("python3")))
        .or_else(|| Some(Cow::from("python")))
}

pub struct PythonEnv {
    venv: Option<PathBuf>,
}

impl Default for PythonEnv {
    fn default() -> Self {
        Self::at(python_env())
    }
}

impl PythonEnv {
    pub fn system() -> Self {
        Self { venv: None }
    }

    pub fn at(path: PathBuf) -> Self {
        Self { venv: Some(path) }
    }

    pub fn path(&self) -> Option<&PathBuf> {
        self.venv.as_ref()
    }

    pub fn exists(&self) -> bool {
        match self.venv {
            None => python().is_some(),
            Some(ref path) => path.is_dir(),
        }
    }

    pub fn create(&self) -> std::io::Result<()> {
        if self.exists() {
            return Ok(());
        }

        let Some(ref path) = self.venv else {
            return Ok(());
        };

        // Create the directory if it doesn't exist:
        std::fs::create_dir_all(path)?;

        // Create the venv if it doesn't exist:
        self.python()
            .args(["-m", "venv", path.to_str().unwrap()])
            .status()?;

        Ok(())
    }

    pub fn python(&self) -> Command {
        match self.venv {
            None => Command::new(python().unwrap().as_ref()),
            Some(ref path) => {
                let mut command = Command::new(path.join("bin/python3"));
                command.env("VIRTUAL_ENV", path.as_os_str());
                command
            }
        }
    }

    pub fn pip(&self) -> Command {
        let mut command = self.python();
        command.args(["-m", "pip"]);
        command
    }

    pub fn pip_command(&self, subcommand: &str, verbosity: u8) -> Command {
        let mut command = self.pip();
        command.arg(subcommand).args(Self::pip_verbosity(verbosity));
        command.args(["--no-input", "--disable-pip-version-check"]);
        command
    }

    /// Returns the verbosity flags for `pip` from a normalized level.
    pub fn pip_verbosity(verbosity: u8) -> Vec<&'static str> {
        match verbosity {
            0 => vec!["-q"],
            1 => vec![],
            2 => vec!["-v"],
            3 => vec!["-vv"],
            _ => vec!["-vvv"],
        }
    }
}

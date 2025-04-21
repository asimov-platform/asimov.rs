// This is free and unencumbered software released into the public domain.

use crate::{env::Env, paths::python_env};
use std::{
    borrow::Cow,
    fs::ReadDir,
    io::Result,
    path::PathBuf,
    process::{Command, ExitStatus},
};

pub struct PythonEnv {
    venv: Option<PathBuf>,
}

impl Default for PythonEnv {
    fn default() -> Self {
        Self::at(python_env())
    }
}

impl Env for PythonEnv {
    // fn is_module_installed(&self, module_name: impl ToString) -> Result<bool> {
    //     let package_name = format!("asimov-{}-module", module_name.to_string());
    //     Ok(self
    //         .pip_command("show", 0)
    //         .args([&package_name])
    //         .status()?
    //         .success())
    // }

    fn path(&self) -> Option<&std::path::PathBuf> {
        self.venv.as_ref()
    }

    fn is_initialized(&self) -> bool {
        match self.venv {
            None => python().is_some(),
            Some(ref path) => path.is_dir(),
        }
    }

    fn initialize(&self) -> Result<()> {
        if self.is_initialized() {
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

    fn available_modules(&self) -> std::io::Result<Vec<String>> {
        Ok(vec![]) // TODO
    }

    fn installed_modules(&self) -> std::io::Result<Vec<String>> {
        let mut result = vec![];
        for entry in self.packages_dir()? {
            let Ok(entry) = entry else {
                continue; // skip invalid entries
            };
            if !entry.path().is_dir() {
                continue; // skip non-directory entries
            }
            let name = entry.file_name();
            let Some(name) = name.to_str() else {
                continue; // skip non-UTF-8 entries
            };
            let Some(name) = name.strip_prefix("asimov_") else {
                continue;
            };
            let Some(pos) = name.find("_module-") else {
                continue;
            };
            result.push(name[..pos].to_string());
        }
        Ok(result)
    }

    fn install_module(
        &self,
        module_name: impl ToString,
        verbosity: Option<u8>,
    ) -> Result<ExitStatus> {
        if !self.is_initialized() {
            self.initialize()?;
        }

        let package_name = format!("asimov-{}-module", module_name.to_string());

        self.pip_command("install", verbosity.unwrap_or(0))
            .args(["--pre", &package_name])
            .status()
    }

    fn uninstall_module(
        &self,
        module_name: impl ToString,
        verbosity: Option<u8>,
    ) -> Result<ExitStatus> {
        if !self.is_initialized() {
            return Ok(ExitStatus::default());
        }

        let package_name = format!("asimov-{}-module", module_name.to_string());

        self.pip_command("uninstall", verbosity.unwrap_or(0))
            .args(["--yes", &package_name])
            .status()
    }
}

impl PythonEnv {
    pub fn system() -> Self {
        Self { venv: None }
    }

    pub fn at(path: PathBuf) -> Self {
        Self { venv: Some(path) }
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

    pub fn packages_dir(&self) -> std::io::Result<ReadDir> {
        let Some(ref path) = self.packages_path() else {
            return Err(std::io::Error::new(
                std::io::ErrorKind::NotFound,
                "`site-packages` directory not found",
            ));
        };
        std::fs::read_dir(path)
    }

    pub fn packages_path(&self) -> Option<PathBuf> {
        self.lib_path().map(|p| p.join("site-packages"))
    }

    pub fn lib_path(&self) -> Option<PathBuf> {
        let Some(ref path) = self.venv else {
            return None;
        };
        // TODO: Improve the check for the Python version
        // See: https://devguide.python.org/versions/#supported-versions
        let path = path.join("lib/python3.13");
        if path.is_dir() {
            return Some(path);
        }
        let path = path.join("lib/python3.12");
        if path.is_dir() {
            return Some(path);
        }
        let path = path.join("lib/python3.11");
        if path.is_dir() {
            return Some(path);
        }
        let path = path.join("lib/python3.10");
        if path.is_dir() {
            return Some(path);
        }
        let path = path.join("lib/python3.9");
        if path.is_dir() {
            return Some(path);
        }
        None // no packages installed
    }
}

pub fn python() -> Option<Cow<'static, str>> {
    getenv::python()
        .map(Cow::from)
        .or_else(|| Some(Cow::from("python3")))
        .or_else(|| Some(Cow::from("python")))
}

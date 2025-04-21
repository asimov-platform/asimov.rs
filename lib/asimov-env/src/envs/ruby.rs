// This is free and unencumbered software released into the public domain.

use crate::{env::Env, paths::ruby_env};
use std::{
    borrow::Cow,
    fs::ReadDir,
    io::Result,
    path::PathBuf,
    process::{Command, ExitStatus},
};

pub struct RubyEnv {
    venv: Option<PathBuf>,
}

impl Default for RubyEnv {
    fn default() -> Self {
        Self::at(ruby_env())
    }
}

impl Env for RubyEnv {
    fn path(&self) -> Option<&std::path::PathBuf> {
        self.venv.as_ref()
    }

    fn is_initialized(&self) -> bool {
        match self.venv {
            None => ruby().is_some(),
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

        Ok(())
    }

    fn available_modules(&self) -> std::io::Result<Vec<String>> {
        Ok(vec![]) // TODO
    }

    fn installed_modules(&self) -> std::io::Result<Vec<String>> {
        let mut result = vec![];
        for entry in self.gems_dir()? {
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
            let Some(name) = name.strip_prefix("asimov-") else {
                continue;
            };
            let Some(pos) = name.find("-module-") else {
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

        self.gem_command("install", verbosity.unwrap_or(0))
            .args(["--prerelease", "--no-document", &package_name])
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

        self.gem_command("uninstall", verbosity.unwrap_or(0))
            .args(["--all", "--executables", &package_name])
            .status()
    }
}

impl RubyEnv {
    pub fn system() -> Self {
        Self { venv: None }
    }

    pub fn at(path: PathBuf) -> Self {
        Self { venv: Some(path) }
    }

    pub fn ruby(&self) -> Command {
        match self.venv {
            None => Command::new(ruby().unwrap().as_ref()),
            Some(ref path) => {
                let venv_ruby = path.join("bin/ruby");
                Command::new(if venv_ruby.is_file() {
                    venv_ruby
                } else {
                    PathBuf::from(ruby().unwrap().as_ref()) // TODO: remove the allocation
                })
            }
        }
    }

    pub fn gem(&self) -> Command {
        let mut command = self.ruby();
        command.args(["-S", "gem"]);
        command
    }

    pub fn gem_command(&self, subcommand: &str, verbosity: u8) -> Command {
        let mut command = self.gem();
        command.arg(subcommand).args(Self::gem_verbosity(verbosity));
        if let Some(ref path) = self.venv {
            command.args(["--install-dir", path.to_str().unwrap()]);
        }
        command
    }

    /// Returns the verbosity flags for `gem` from a normalized level.
    pub fn gem_verbosity(verbosity: u8) -> Vec<&'static str> {
        match verbosity {
            0 => vec!["--silent"],
            1 => vec!["--quiet"], // -q
            2 => vec![],
            _ => vec!["--verbose"], // -V
        }
    }

    pub fn gems_dir(&self) -> std::io::Result<ReadDir> {
        let Some(ref path) = self.gems_path() else {
            return Err(std::io::Error::new(
                std::io::ErrorKind::NotFound,
                "`gems` directory not found",
            ));
        };
        std::fs::read_dir(path)
    }

    pub fn gems_path(&self) -> Option<PathBuf> {
        let Some(ref path) = self.venv else {
            return None;
        };
        let path = path.join("gems");
        if !path.is_dir() {
            return None; // no gems installed
        }
        Some(path)
    }
}

pub fn ruby() -> Option<Cow<'static, str>> {
    getenv::ruby()
        .map(Cow::from)
        .or_else(|| Some(Cow::from("ruby")))
}

pub fn gem() -> Option<Cow<'static, str>> {
    getenv::gem()
        .map(Cow::from)
        .or_else(|| Some(Cow::from("gem")))
}

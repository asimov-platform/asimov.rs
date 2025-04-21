// This is free and unencumbered software released into the public domain.

use crate::paths::ruby_env;
use std::{borrow::Cow, path::PathBuf, process::Command};

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

pub struct RubyEnv {
    venv: Option<PathBuf>,
}

impl Default for RubyEnv {
    fn default() -> Self {
        Self::at(ruby_env())
    }
}

impl RubyEnv {
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
            None => ruby().is_some(),
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

        Ok(())
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
}

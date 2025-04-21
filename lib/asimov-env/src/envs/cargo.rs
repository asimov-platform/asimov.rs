// This is free and unencumbered software released into the public domain.

use crate::env::Env;
use std::{
    borrow::Cow,
    io::{BufRead, BufReader, Result},
    process::{Command, ExitStatus, Stdio},
};

#[derive(Default)]
pub struct CargoEnv {}

impl Env for CargoEnv {
    fn path(&self) -> Option<&std::path::PathBuf> {
        None
    }

    fn is_initialized(&self) -> bool {
        true // nothing needed
    }

    fn initialize(&self) -> Result<()> {
        Ok(()) // nothing needed
    }

    fn available_modules(&self) -> std::io::Result<Vec<String>> {
        Ok(vec![]) // TODO
    }

    fn installed_modules(&self) -> std::io::Result<Vec<String>> {
        if !self.is_initialized() {
            return Ok(vec![]);
        }

        let mut result = vec![];

        let output = Command::new(cargo().unwrap().as_ref())
            .args(["install", "--list", "--quiet", "--color=never"])
            .stdout(Stdio::piped())
            .stderr(Stdio::null())
            .spawn()?
            .stdout
            .ok_or_else(|| {
                std::io::Error::new(std::io::ErrorKind::Other, "failed to capture stdout")
            })?;

        for line in BufReader::new(output).lines() {
            let line = line?;
            if line.starts_with("asimov-") && line.contains("-module v") && line.ends_with(':') {
                let crate_name = line.split_whitespace().next().unwrap_or("");
                let module_name = crate_name.strip_prefix("asimov-").unwrap();
                let Some(module_name) = module_name.strip_suffix("-module") else {
                    continue;
                };
                result.push(module_name.to_string());
            }
        }

        Ok(result)
    }

    fn install_module(
        &self,
        module_name: impl ToString,
        _verbosity: Option<u8>,
    ) -> Result<ExitStatus> {
        if !self.is_initialized() {
            self.initialize()?;
        }

        let package_name = format!("asimov-{}-module", module_name.to_string());

        Command::new(cargo().unwrap().as_ref())
            .args(["install", &package_name])
            .status()
    }

    fn uninstall_module(
        &self,
        module_name: impl ToString,
        _verbosity: Option<u8>,
    ) -> Result<ExitStatus> {
        if !self.is_initialized() {
            return Ok(ExitStatus::default());
        }

        let package_name = format!("asimov-{}-module", module_name.to_string());

        Command::new(cargo().unwrap().as_ref())
            .args(["uninstall", &package_name])
            .status()
    }
}

impl CargoEnv {
    pub fn system() -> Self {
        Self::default()
    }
}

pub fn cargo() -> Option<Cow<'static, str>> {
    getenv::cargo()
        .map(Cow::from)
        .or_else(|| Some(Cow::from("cargo")))
}

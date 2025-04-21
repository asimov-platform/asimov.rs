// This is free and unencumbered software released into the public domain.

#[cfg(feature = "std")]
use std::{io::Result, process::ExitStatus};

#[cfg(not(feature = "std"))]
type Result<T> = core::result::Result<T, Box<dyn core::error::Error>>;

pub trait Env {
    fn is_module_available(&self, module_name: impl ToString) -> Result<bool> {
        Ok(self.available_modules()?.contains(&module_name.to_string()))
    }

    fn is_module_installed(&self, module_name: impl ToString) -> Result<bool> {
        if !self.is_initialized() {
            return Ok(false);
        }
        Ok(self.installed_modules()?.contains(&module_name.to_string()))
    }

    fn is_module_enabled(&self, _module_name: impl ToString) -> Result<bool> {
        Ok(true) // TODO
    }

    #[cfg(feature = "std")]
    fn path(&self) -> Option<&std::path::PathBuf> {
        None
    }

    fn is_initialized(&self) -> bool {
        false
    }

    fn initialize(&self) -> Result<()> {
        Ok(())
    }

    fn available_modules(&self) -> std::io::Result<Vec<String>> {
        Ok(vec![])
    }

    fn installed_modules(&self) -> std::io::Result<Vec<String>> {
        Ok(vec![])
    }

    #[cfg(feature = "std")]
    fn install_module(
        &self,
        _module_name: impl ToString,
        _verbosity: Option<u8>,
    ) -> Result<ExitStatus> {
        Ok(ExitStatus::default())
    }

    #[cfg(feature = "std")]
    fn uninstall_module(
        &self,
        _module_name: impl ToString,
        _verbosity: Option<u8>,
    ) -> Result<ExitStatus> {
        Ok(ExitStatus::default())
    }
}

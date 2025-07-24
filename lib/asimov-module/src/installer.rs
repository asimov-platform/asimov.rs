// This is free and unencumbered software released into the public domain.

use std::{string::String, vec::Vec};

use crate::{models::InstalledModuleManifest, tracing};

pub mod error;
use error::*;

#[derive(Clone, Debug)]
pub struct Installer {
    client: reqwest::Client,
    dir: std::path::PathBuf,
}

impl Default for Installer {
    fn default() -> Self {
        // TODO: retry support
        let client = reqwest::Client::builder()
            .user_agent("asimov-module-installer")
            .timeout(std::time::Duration::from_secs(10))
            .build()
            .expect("Failed to build HTTP client");
        let dir = asimov_env::paths::asimov_root();
        Self::new(client, dir)
    }
}

impl Installer {
    pub fn new(client: reqwest::Client, directory: impl Into<std::path::PathBuf>) -> Self {
        Self {
            client,
            dir: directory.into(),
        }
    }

    pub async fn installed_modules(&self) -> Result<Vec<InstalledModuleManifest>, ReadError> {
        todo!();
    }

    pub async fn enabled_modules(&self) -> Result<Vec<InstalledModuleManifest>, ReadError> {
        todo!();
    }

    pub async fn is_module_installed(
        &self,
        _module_name: impl AsRef<str>,
    ) -> Result<bool, ReadError> {
        todo!();
    }

    pub async fn is_module_enabled(
        &self,
        _module_name: impl AsRef<str>,
    ) -> Result<bool, ReadError> {
        todo!();
    }

    pub async fn fetch_latest_release(
        &self,
        _module_name: impl AsRef<str>,
    ) -> Result<String, FetchError> {
        todo!();
    }

    pub async fn module_version(
        &self,
        _module_name: impl AsRef<str>,
    ) -> Result<Option<String>, ReadError> {
        todo!();
    }

    pub async fn install_module(
        &self,
        _module_name: impl AsRef<str>,
        _version: impl AsRef<str>,
    ) -> Result<(), InstallError> {
        todo!();
    }

    pub async fn uninstall_module(
        &self,
        _module_name: impl AsRef<str>,
    ) -> Result<(), UninstallError> {
        todo!();
    }

    pub async fn enable_module(&self, _module_name: impl AsRef<str>) -> Result<(), EnableError> {
        todo!();
    }

    pub async fn disable_module(&self, _module_name: impl AsRef<str>) -> Result<(), DisableError> {
        todo!();
    }
}

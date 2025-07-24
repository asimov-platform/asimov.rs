// This is free and unencumbered software released into the public domain.

use std::{
    borrow::ToOwned,
    path::{Path, PathBuf},
    string::String,
    vec::Vec,
};

use crate::{models::InstalledModuleManifest, tracing};

pub mod error;
use error::*;
use tokio::io;

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
    pub fn new(client: reqwest::Client, asimov_dir: impl Into<std::path::PathBuf>) -> Self {
        Self {
            client,
            dir: asimov_dir.into(),
        }
    }

    pub async fn installed_modules(&self) -> Result<Vec<InstalledModuleManifest>, ReadError> {
        let installed_dir = self.install_dir();

        let mut read_dir =
            tokio::fs::read_dir(&installed_dir)
                .await
                .map_err(|e| ReadError::InstallDirIo {
                    path: installed_dir.clone(),
                    source: e,
                })?;

        let mut modules = Vec::new();

        while let Some(entry) =
            read_dir
                .next_entry()
                .await
                .map_err(|e| ReadError::InstallDirIo {
                    path: installed_dir.clone(),
                    source: e,
                })?
        {
            let path = entry.path();

            if !tokio::fs::metadata(&path)
                .await
                .map(|md| md.is_file())
                .unwrap_or(false)
            {
                continue;
            }

            let manifest = read_manifest(&path).await?;

            modules.push(manifest)
        }

        Ok(modules)
    }

    pub async fn enabled_modules(&self) -> Result<Vec<InstalledModuleManifest>, ReadError> {
        let enabled_dir = self.enable_dir();

        let mut read_dir =
            tokio::fs::read_dir(&enabled_dir)
                .await
                .map_err(|e| ReadError::InstallDirIo {
                    path: enabled_dir.clone(),
                    source: e,
                })?;

        let mut modules = Vec::new();

        while let Some(entry) = read_dir
            .next_entry()
            .await
            .map_err(|e| ReadError::EnableDirIo {
                path: enabled_dir.clone(),
                source: e,
            })?
        {
            let path = entry.path();

            if !tokio::fs::metadata(&path)
                .await
                .map(|md| md.is_symlink())
                .unwrap_or(false)
            {
                continue;
            }

            let manifest_path = tokio::fs::read_link(&path).await.unwrap();

            let manifest = read_manifest(&manifest_path).await?;

            modules.push(manifest)
        }

        Ok(modules)
    }

    pub async fn is_module_installed(
        &self,
        module_name: impl AsRef<str>,
    ) -> Result<bool, ReadManifestError> {
        self.find_manifest_file(module_name)
            .await
            .map(|path| path.is_some())
    }

    pub async fn is_module_enabled(&self, module_name: impl AsRef<str>) -> Result<bool, ReadError> {
        let path = self.enable_dir().join(module_name.as_ref());

        tokio::fs::metadata(&path)
            .await
            .map(|md| md.is_symlink())
            .map_err(|e| ReadError::EnabledLinkIo { path, source: e })
    }

    pub async fn fetch_latest_release(
        &self,
        _module_name: impl AsRef<str>,
    ) -> Result<String, FetchError> {
        todo!();
    }

    pub async fn module_version(
        &self,
        module_name: impl AsRef<str>,
    ) -> Result<Option<String>, ReadManifestError> {
        let Some(path) = self.find_manifest_file(module_name).await? else {
            return Ok(None);
        };

        let manifest = read_manifest(&path).await?;

        Ok(manifest.version)
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

    async fn find_manifest_file(
        &self,
        module_name: impl AsRef<str>,
    ) -> Result<Option<PathBuf>, ReadManifestError> {
        let install_dir = self.install_dir();

        let module_name = module_name.as_ref();

        let files = [
            std::format!("{module_name}.json"),
            std::format!("{module_name}.yaml"),
            std::format!("{module_name}.yml"),
        ];

        for file in files {
            let path = install_dir.join(file);
            match tokio::fs::try_exists(&path).await {
                Ok(exists) if exists => return Ok(Some(path)),
                Err(err) if err.kind() != io::ErrorKind::NotFound => {
                    return Err(ReadManifestError::InstalledManifestIo { path, source: err });
                },
                _ => continue,
            }
        }

        Ok(None)
    }

    fn install_dir(&self) -> PathBuf {
        self.dir.join("modules").join("installed")
    }

    fn enable_dir(&self) -> PathBuf {
        self.dir.join("modules").join("enabled")
    }

    fn exec_dir(&self) -> PathBuf {
        self.dir.join("libexec")
    }
}

async fn read_manifest(
    path: impl AsRef<Path>,
) -> Result<InstalledModuleManifest, ReadManifestError> {
    match path.as_ref().extension().and_then(|ext| ext.to_str()) {
        Some("yaml") | Some("yml") => {
            let content = tokio::fs::read(&path).await.map_err(|e| {
                ReadManifestError::InstalledManifestIo {
                    path: path.as_ref().to_owned(),
                    source: e,
                }
            })?;

            serde_yml::from_slice::<'_, InstalledModuleManifest>(&content).map_err(|e| {
                ReadManifestError::ManifestDeserialize {
                    path: path.as_ref().to_owned(),
                    source: e.into(),
                }
            })
        },
        Some("json") => {
            let content = tokio::fs::read(&path).await.map_err(|e| {
                ReadManifestError::InstalledManifestIo {
                    path: path.as_ref().to_owned(),
                    source: e,
                }
            })?;

            serde_yml::from_slice::<'_, InstalledModuleManifest>(&content).map_err(|e| {
                ReadManifestError::ManifestDeserialize {
                    path: path.as_ref().to_owned(),
                    source: e.into(),
                }
            })
        },
        ext => Err(ReadManifestError::UnknownManifestFormat {
            path: path.as_ref().to_owned(),
            extension: ext.map(Into::into),
        }),
    }
}

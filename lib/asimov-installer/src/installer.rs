// This is free and unencumbered software released into the public domain.

use asimov_module::{InstalledModuleManifest, ModuleManifest, tracing};
use std::{path::Path, string::String};
use tokio::io;

pub mod error;
use error::*;

use asimov_registry::{Registry, error::ReadManifestError};

mod github;
mod platform;

#[derive(Clone, Debug)]
pub struct Installer {
    client: reqwest::Client,
    registry: Registry,
}

impl Default for Installer {
    fn default() -> Self {
        // TODO: retry support
        let client = reqwest::Client::builder()
            .user_agent("asimov-module-installer")
            .timeout(std::time::Duration::from_secs(10))
            .build()
            .expect("Failed to build HTTP client");
        let registry = Registry::default();
        Self::new(client, registry)
    }
}

impl Installer {
    pub fn new(client: reqwest::Client, registry: Registry) -> Self {
        Self { client, registry }
    }

    pub async fn install_module(
        &self,
        module_name: impl AsRef<str>,
        version: impl AsRef<str>,
    ) -> Result<(), InstallError> {
        let temp_dir = tempfile::Builder::new()
            .prefix("asimov-module-installer")
            .tempdir()
            .map_err(InstallError::CreateTempDir)?;

        let manifest = self
            .preinstall(module_name.as_ref(), version.as_ref(), temp_dir.path())
            .await?;

        self.finish_install(
            module_name.as_ref(),
            version.as_ref(),
            manifest,
            temp_dir.path(),
        )
        .await?;

        Ok(())
    }

    pub async fn fetch_latest_release(
        &self,
        module_name: impl AsRef<str>,
    ) -> Result<String, FetchError> {
        github::fetch_latest_release(&self.client, module_name).await
    }

    pub async fn upgrade_module(
        &self,
        module_name: impl AsRef<str>,
        version: impl AsRef<str>,
    ) -> Result<(), UpgradeError> {
        let module_name = module_name.as_ref();
        let version = version.as_ref();

        let current_version = self.registry.module_version(module_name).await?;
        match current_version {
            Some(current) if current == version => return Ok(()),
            Some(_) => (),
            None => tracing::debug!(module_name, "installed module does not define a version"),
        };

        let temp_dir = tempfile::Builder::new()
            .prefix("asimov-module-installer")
            .tempdir()
            .map_err(UpgradeError::CreateTempDir)?;

        // check if currently enabled, have to re-enable after upgrade
        let was_enabled = self.registry.is_module_enabled(module_name).await?;

        let manifest = self
            .preinstall(module_name, version, temp_dir.path())
            .await?;

        // now ok to uninstall old version
        self.uninstall_module(module_name).await?;

        self.finish_install(module_name, version, manifest, temp_dir.path())
            .await?;

        if was_enabled {
            self.registry.enable_module(module_name).await?;
        }

        Ok(())
    }

    pub async fn uninstall_module(
        &self,
        module_name: impl AsRef<str>,
    ) -> Result<(), UninstallError> {
        let manifest_path = self
            .registry
            .find_manifest_file(module_name.as_ref())
            .await?
            .ok_or(UninstallError::NotInstalled)?;

        let manifest = read_manifest(&manifest_path).await?;

        self.registry.disable_module(&module_name).await?;

        tokio::fs::remove_file(&manifest_path).await.or_else(|e| {
            if e.kind() == io::ErrorKind::NotFound {
                Ok(())
            } else {
                Err(UninstallError::Delete(manifest_path, e))
            }
        })?;

        let exec_dir = self.registry.exec_dir();

        for program in manifest.manifest.provides.programs {
            let path = exec_dir.join(program);
            tokio::fs::remove_file(&path).await.or_else(|e| {
                if e.kind() == io::ErrorKind::NotFound {
                    Ok(())
                } else {
                    Err(UninstallError::Delete(path, e))
                }
            })?;
        }

        Ok(())
    }

    async fn preinstall(
        &self,
        module_name: &str,
        version: &str,
        temp_dir: &Path,
    ) -> Result<ModuleManifest, PreinstallError> {
        let platform = platform::detect_platform();

        let release = github::fetch_release(&self.client, module_name, version)
            .await
            .map_err(|_| {
                PreinstallError::CreateManifestDir(std::io::Error::other("Failed to fetch release"))
            })?;

        let Some(asset) = github::find_matching_asset(&release.assets, module_name, &platform)
        else {
            return Err(PreinstallError::NotAvailable(platform));
        };

        let manifest = github::fetch_module_manifest(&self.client, module_name, version)
            .await
            .map_err(PreinstallError::FetchManifest)?;

        let download = github::download_asset(&self.client, asset, temp_dir).await?;

        match github::fetch_checksum(&self.client, asset).await {
            Ok(None) => {},
            Ok(Some(checksum)) => {
                github::verify_checksum(&download, &checksum).await?;
            },
            Err(err) => Err(err)?,
        }

        let extract_dir = temp_dir.join("extract");
        tokio::fs::create_dir(&extract_dir)
            .await
            .map_err(PreinstallError::CreateExtractDir)?;

        github::extract_files(&download, &extract_dir)
            .await
            .map_err(PreinstallError::Extract)?;

        Ok(manifest)
    }

    async fn finish_install(
        &self,
        module_name: &str,
        version: &str,
        manifest: ModuleManifest,
        temp_dir: &Path,
    ) -> Result<(), FinishInstallError> {
        let extract_dir = temp_dir.join("extract");

        let manifest_path = self
            .registry
            .install_dir()
            .join(std::format!("{module_name}.json"));

        github::install_binaries(&manifest, &extract_dir, &self.registry.exec_dir())
            .await
            .map_err(FinishInstallError::BinaryInstall)?;

        let installed_manifest = InstalledModuleManifest {
            version: Some(version.into()),
            manifest,
        };
        let mut manifest_json = serde_json::to_string_pretty(&installed_manifest)?;
        manifest_json.push('\n'); // always newline-terminate text files

        tokio::fs::write(&manifest_path, &manifest_json)
            .await
            .map_err(FinishInstallError::SaveManifest)?;

        Ok(())
    }
}

async fn read_manifest(
    path: impl AsRef<Path>,
) -> Result<InstalledModuleManifest, ReadManifestError> {
    let manifest = match path.as_ref().extension().and_then(|ext| ext.to_str()) {
        Some("yaml") | Some("yml") => {
            let content = tokio::fs::read(&path)
                .await
                .map_err(ReadManifestError::InstalledManifestIo)?;

            serde_yaml_ng::from_slice::<'_, InstalledModuleManifest>(&content)?
        },
        Some("json") => {
            let content = tokio::fs::read(&path)
                .await
                .map_err(ReadManifestError::InstalledManifestIo)?;

            serde_json::from_slice::<'_, InstalledModuleManifest>(&content)?
        },
        ext => Err(ReadManifestError::UnknownManifestFormat(
            ext.map(Into::into),
        ))?,
    };
    Ok(manifest)
}

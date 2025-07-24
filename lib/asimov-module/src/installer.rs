// This is free and unencumbered software released into the public domain.

use std::{
    path::{Path, PathBuf},
    string::String,
    vec::Vec,
};
use tokio::io;

use crate::{models::InstalledModuleManifest, tracing};

pub mod error;
use error::*;

mod github;

mod platform;

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

    pub async fn create_file_tree(&self) -> Result<(), CreateFileTreeError> {
        let install_dir = self.install_dir();
        tokio::fs::create_dir_all(&install_dir)
            .await
            .map_err(|e| CreateFileTreeError::InstallDir(install_dir, e))?;

        let enable_dir = self.enable_dir();
        tokio::fs::create_dir_all(&enable_dir)
            .await
            .map_err(|e| CreateFileTreeError::EnableDir(enable_dir, e))?;

        let exec_dir = self.exec_dir();
        tokio::fs::create_dir_all(&exec_dir)
            .await
            .map_err(|e| CreateFileTreeError::ExecDir(exec_dir, e))?;

        Ok(())
    }

    pub async fn installed_modules(&self) -> Result<Vec<InstalledModuleManifest>, ReadError> {
        let installed_dir = self.install_dir();

        let mut read_dir = tokio::fs::read_dir(&installed_dir)
            .await
            .map_err(|e| ReadError::InstallDirIo(installed_dir.clone(), e))?;

        let mut modules = Vec::new();

        while let Some(entry) = read_dir
            .next_entry()
            .await
            .map_err(|e| ReadError::InstallDirIo(installed_dir.clone(), e))?
        {
            let path = entry.path();

            if !tokio::fs::metadata(&path)
                .await
                .map(|md| md.is_file())
                .unwrap_or(false)
            {
                continue;
            }

            let manifest = read_manifest(&path)
                .await
                .map_err(|e| ReadError::ReadManifestError(path, e))?;

            modules.push(manifest)
        }

        Ok(modules)
    }

    pub async fn enabled_modules(&self) -> Result<Vec<InstalledModuleManifest>, ReadError> {
        let enabled_dir = self.enable_dir();

        let mut read_dir = tokio::fs::read_dir(&enabled_dir)
            .await
            .map_err(|e| ReadError::InstallDirIo(enabled_dir.clone(), e))?;

        let mut modules = Vec::new();

        while let Some(entry) = read_dir
            .next_entry()
            .await
            .map_err(|e| ReadError::EnableDirIo(enabled_dir.clone(), e))?
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

            let manifest = read_manifest(&manifest_path)
                .await
                .map_err(|e| ReadError::ReadManifestError(path, e))?;

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
            .map_err(|e| ReadError::EnabledLinkIo(path, e))
    }

    pub async fn fetch_latest_release(
        &self,
        module_name: impl AsRef<str>,
    ) -> Result<String, FetchError> {
        github::fetch_latest_release(&self.client, module_name).await
    }

    pub async fn module_version(
        &self,
        module_name: impl AsRef<str>,
    ) -> Result<Option<String>, ReadModuleVersionError> {
        let path = self
            .find_manifest_file(module_name)
            .await?
            .ok_or(ReadModuleVersionError::NotInstalled)?;

        let manifest = read_manifest(&path).await?;

        Ok(manifest.version)
    }

    pub async fn install_module(
        &self,
        module_name: impl AsRef<str>,
        version: impl AsRef<str>,
    ) -> Result<(), InstallError> {
        let platform = platform::detect_platform();

        let release = github::fetch_release(&self.client, module_name.as_ref(), version.as_ref())
            .await
            .unwrap();

        let Some(asset) =
            github::find_matching_asset(&release.assets, module_name.as_ref(), &platform)
        else {
            return Err(InstallError::NotAvailable(platform));
        };

        let manifest =
            github::fetch_module_manifest(&self.client, module_name.as_ref(), version.as_ref())
                .await
                .map_err(InstallError::FetchManifest)?;

        let temp_dir = tempfile::Builder::new()
            .prefix("asimov-module-installer")
            .tempdir()
            .map_err(InstallError::CreateTempDir)?;

        let download = github::download_asset(&self.client, asset, temp_dir.path()).await?;

        match github::fetch_checksum(&self.client, asset).await {
            Ok(None) => {},
            Ok(Some(checksum)) => {
                github::verify_checksum(&download, &checksum).await?;
            },
            Err(err) => Err(err)?,
        }

        let extract_dir = temp_dir.path().join("extract");
        tokio::fs::create_dir(&extract_dir)
            .await
            .map_err(InstallError::CreateExtractDir)?;

        github::extract_files(&download, &extract_dir)
            .await
            .map_err(InstallError::Extract)?;

        let manifest_path = self
            .install_dir()
            .join(std::format!("{}.json", module_name.as_ref()));

        github::install_binaries(&manifest, &extract_dir, &self.exec_dir())
            .await
            .map_err(InstallError::BinaryInstall)?;

        let manifest = InstalledModuleManifest {
            version: Some(version.as_ref().into()),
            manifest,
        };
        let manifest = serde_json::to_string_pretty(&manifest)?;

        tokio::fs::write(&manifest_path, &manifest)
            .await
            .map_err(InstallError::SaveManifest)?;

        Ok(())
    }

    pub async fn upgrade_module(
        &self,
        module_name: impl AsRef<str>,
        version: impl AsRef<str>,
    ) -> Result<(), UpgradeError> {
        let module_name = module_name.as_ref();
        let version = version.as_ref();

        if !self.is_module_installed(module_name).await? {
            return Err(UpgradeError::NotInstalled);
        }

        // check if currently enabled, have to re-enable after upgrade
        let was_enabled = self.is_module_enabled(module_name).await.unwrap_or(false);

        let platform = platform::detect_platform();

        let release = github::fetch_release(&self.client, module_name, version)
            .await
            .map_err(|e| {
                UpgradeError::Predownload(InstallError::CreateManifestDir(std::io::Error::other(
                    std::format!("Failed to fetch release: {}", e),
                )))
            })?;

        let Some(asset) = github::find_matching_asset(&release.assets, module_name, &platform)
        else {
            return Err(UpgradeError::Predownload(InstallError::NotAvailable(
                platform,
            )));
        };

        let manifest = github::fetch_module_manifest(&self.client, module_name, version)
            .await
            .map_err(|e| UpgradeError::Predownload(InstallError::FetchManifest(e)))?;

        let temp_dir = tempfile::Builder::new()
            .prefix("asimov-module-installer")
            .tempdir()
            .map_err(|e| UpgradeError::Predownload(InstallError::CreateTempDir(e)))?;

        let download = github::download_asset(&self.client, asset, temp_dir.path())
            .await
            .map_err(|e| UpgradeError::Predownload(InstallError::Download(e)))?;

        match github::fetch_checksum(&self.client, asset).await {
            Ok(None) => {},
            Ok(Some(checksum)) => {
                github::verify_checksum(&download, &checksum)
                    .await
                    .map_err(|e| UpgradeError::Predownload(InstallError::VerifyChecksum(e)))?;
            },
            Err(err) => {
                return Err(UpgradeError::Predownload(InstallError::FetchChecksum(err)));
            },
        }

        let extract_dir = temp_dir.path().join("extract");
        tokio::fs::create_dir(&extract_dir)
            .await
            .map_err(|e| UpgradeError::Predownload(InstallError::CreateExtractDir(e)))?;

        github::extract_files(&download, &extract_dir)
            .await
            .map_err(|e| UpgradeError::Predownload(InstallError::Extract(e)))?;

        // now ok to uninstall old version
        self.uninstall_module(module_name).await?;

        let manifest_path = self
            .install_dir()
            .join(std::format!("{}.json", module_name));

        github::install_binaries(&manifest, &extract_dir, &self.exec_dir())
            .await
            .map_err(|e| UpgradeError::Install(InstallError::BinaryInstall(e)))?;

        let installed_manifest = InstalledModuleManifest {
            version: Some(version.into()),
            manifest,
        };
        let manifest_json = serde_json::to_string_pretty(&installed_manifest)
            .map_err(|e| UpgradeError::Install(InstallError::SerializeManifest(e)))?;

        tokio::fs::write(&manifest_path, &manifest_json)
            .await
            .map_err(|e| UpgradeError::Install(InstallError::SaveManifest(e)))?;

        // Re-enable the module if it was previously enabled
        if was_enabled {
            self.enable_module(module_name).await.map_err(|e| {
                UpgradeError::Install(InstallError::CreateManifestDir(std::io::Error::other(
                    std::format!("Failed to re-enable module: {}", e),
                )))
            })?;
        }

        Ok(())
    }

    pub async fn uninstall_module(
        &self,
        module_name: impl AsRef<str>,
    ) -> Result<(), UninstallError> {
        let manifest_path = self
            .find_manifest_file(module_name.as_ref())
            .await?
            .ok_or(UninstallError::NotInstalled)?;

        let manifest = read_manifest(&manifest_path).await?;

        self.disable_module(&module_name)
            .await
            .map_err(|DisableError::Io(e)| {
                UninstallError::Io(self.enable_dir().join(module_name.as_ref()), e)
            })?;

        tokio::fs::remove_file(&manifest_path).await.or_else(|e| {
            if e.kind() == io::ErrorKind::NotFound {
                Ok(())
            } else {
                Err(UninstallError::Io(manifest_path, e))
            }
        })?;

        let exec_dir = self.exec_dir();

        for program in manifest.manifest.provides.programs {
            let path = exec_dir.join(program);
            tokio::fs::remove_file(&path).await.or_else(|e| {
                if e.kind() == io::ErrorKind::NotFound {
                    Ok(())
                } else {
                    Err(UninstallError::Io(path, e))
                }
            })?;
        }

        Ok(())
    }

    pub async fn enable_module(&self, module_name: impl AsRef<str>) -> Result<(), EnableError> {
        let target_path = self
            .find_manifest_file(&module_name)
            .await?
            .ok_or(EnableError::NotInstalled)?;

        let src_path = self.enable_dir().join(module_name.as_ref());

        match tokio::fs::symlink(&target_path, &src_path).await {
            Ok(_) => Ok(()),
            Err(err) if err.kind() == io::ErrorKind::AlreadyExists => {
                // disable and retry enabling one more time
                let _ = self.disable_module(module_name).await;
                tokio::fs::symlink(&target_path, &src_path)
                    .await
                    .map_err(EnableError::Io)
            },
            Err(err) => Err(EnableError::Io(err)),
        }
    }

    pub async fn disable_module(&self, module_name: impl AsRef<str>) -> Result<(), DisableError> {
        let path = self.enable_dir().join(module_name.as_ref());

        tokio::fs::remove_file(&path)
            .await
            .or_else(|err| {
                if err.kind() == io::ErrorKind::NotFound {
                    Ok(())
                } else {
                    Err(err)
                }
            })
            .map_err(Into::into)
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

        for file in &files {
            let path = install_dir.join(file);
            match tokio::fs::try_exists(&path).await {
                Ok(exists) if exists => return Ok(Some(path)),
                Err(err) if err.kind() != io::ErrorKind::NotFound => {
                    return Err(ReadManifestError::InstalledManifestIo(err));
                },
                _ => continue,
            }
        }

        let legacy_dir = install_dir
            .parent()
            .expect("should never panic, self.install_dir() has >=2 path segments");
        for file in &files {
            let path = legacy_dir.join(file);
            match tokio::fs::try_exists(&path).await {
                Ok(exists) if exists => {
                    let dst = install_dir.join(file);
                    return Ok(tokio::fs::rename(&path, &dst)
                        .await
                        .inspect_err(|err| {
                            tracing::debug!(
                                from = ?path,
                                to = ?dst,
                                ?err,
                                "tried to move module manifest from legacy path but failed"
                            )
                        })
                        .is_ok()
                        .then(|| dst)
                        .or(Some(path)));
                },
                Err(err) if err.kind() != io::ErrorKind::NotFound => {
                    return Err(ReadManifestError::InstalledManifestIo(err));
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
    let manifest = match path.as_ref().extension().and_then(|ext| ext.to_str()) {
        Some("yaml") | Some("yml") => {
            let content = tokio::fs::read(&path)
                .await
                .map_err(ReadManifestError::InstalledManifestIo)?;

            serde_yml::from_slice::<'_, InstalledModuleManifest>(&content)?
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

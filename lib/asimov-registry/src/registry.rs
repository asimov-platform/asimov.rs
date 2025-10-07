// This is free and unencumbered software released into the public domain.

use alloc::{string::String, vec::Vec};
use asimov_module::{InstalledModuleManifest, tracing};
use std::path::{Path, PathBuf};
use tokio::io;

pub mod error;
use error::*;

#[derive(Clone, Debug, bon::Builder)]
pub struct Options {
    /// Controls whether to search for module manifests from a legacy location.
    /// The legacy (previous) location by default is `~/.asimov/modules/*.yaml`.
    #[builder(default = true)]
    pub search_legacy_path: bool,

    /// Controls whether to automatically move module manifests from a legacy location.
    /// The legacy (previous) location by default is `~/.asimov/modules/*.yaml`.
    /// The new and current location by default is `~/.asimov/modules/installed/*.{yaml,json}`.
    #[builder(default = true)]
    pub auto_migrate_legacy_path: bool,
}

impl Default for Options {
    fn default() -> Self {
        Self {
            search_legacy_path: true,
            auto_migrate_legacy_path: true,
        }
    }
}

#[derive(Clone, Debug)]
pub struct Registry {
    install_dir: PathBuf,
    enable_dir: PathBuf,
    exec_dir: PathBuf,
    legacy_modules_dir: Option<PathBuf>,
    options: Options,
}

impl Default for Registry {
    fn default() -> Self {
        let dir = asimov_env::paths::asimov_root();
        let options = Options::default();
        Self::new(dir, options)
    }
}

impl Registry {
    pub fn new(asimov_dir: impl Into<PathBuf>, options: Options) -> Self {
        let dir = asimov_dir.into();
        Self {
            install_dir: dir.join("modules").join("installed"),
            enable_dir: dir.join("modules").join("enabled"),
            exec_dir: dir.join("libexec"),
            legacy_modules_dir: Some(dir.join("modules")),
            options,
        }
    }

    pub fn with_dirs<S1, S2, S3>(
        install_dir: S1,
        enable_dir: S2,
        exec_dir: S3,
        options: Options,
    ) -> Self
    where
        S1: Into<PathBuf>,
        S2: Into<PathBuf>,
        S3: Into<PathBuf>,
    {
        Self {
            install_dir: install_dir.into(),
            enable_dir: enable_dir.into(),
            exec_dir: exec_dir.into(),
            legacy_modules_dir: None,
            options,
        }
    }

    pub async fn create_file_tree(&self) -> Result<(), CreateFileTreeError> {
        tokio::fs::create_dir_all(&self.install_dir)
            .await
            .map_err(|e| CreateFileTreeError::InstallDir(self.install_dir.clone(), e))?;

        tokio::fs::create_dir_all(&self.enable_dir)
            .await
            .map_err(|e| CreateFileTreeError::EnableDir(self.enable_dir.clone(), e))?;

        tokio::fs::create_dir_all(&self.exec_dir)
            .await
            .map_err(|e| CreateFileTreeError::ExecDir(self.exec_dir.clone(), e))?;

        Ok(())
    }

    pub async fn add_manifest(
        &self,
        manifest: InstalledModuleManifest,
    ) -> Result<(), AddManifestError> {
        let module_name = &manifest.manifest.name;

        if self.is_module_installed(module_name).await.unwrap_or(false) {
            return Err(AddManifestError::AlreadyInstalled);
        }

        let manifest_path = self.install_dir.join(module_name).with_extension("json");

        let serialized = serde_json::to_vec_pretty(&manifest).map_err(SerializeError::Json)?;

        tokio::fs::write(&manifest_path, serialized)
            .await
            .map_err(|e| AddManifestError::WriteManifest(manifest_path, e))
    }

    pub async fn read_manifest(
        &self,
        module_name: impl AsRef<str>,
    ) -> Result<InstalledModuleManifest, ManifestError> {
        let path = self
            .find_manifest_file(module_name)
            .await?
            .ok_or(ManifestError::NotInstalled)?;
        read_manifest(path).await.map_err(Into::into)
    }

    pub async fn module_version(
        &self,
        module_name: impl AsRef<str>,
    ) -> Result<Option<String>, ModuleVersionError> {
        self.read_manifest(module_name)
            .await
            .map(|manifest| manifest.version)
            .map_err(Into::into)
    }

    pub async fn remove_manifest(
        &self,
        module_name: impl AsRef<str>,
    ) -> Result<(), RemoveManifestError> {
        let manifest_path = self
            .find_manifest_file(&module_name)
            .await?
            .ok_or(RemoveManifestError::NotInstalled)?;

        tokio::fs::remove_file(&manifest_path)
            .await
            .map_err(|e| RemoveManifestError::RemoveManifest(manifest_path, e))
    }

    pub async fn add_binary(
        &self,
        name: impl AsRef<str>,
        path: impl AsRef<Path>,
    ) -> Result<(), AddBinaryError> {
        let source_path = path.as_ref();
        let target_path = self.exec_dir.join(name.as_ref());

        if tokio::fs::try_exists(&target_path).await.unwrap_or(false) {
            return Err(AddBinaryError::AlreadyExists);
        }

        tokio::fs::copy(source_path, &target_path)
            .await
            .map_err(AddBinaryError::Copy)?;

        // Make binary executable on Unix systems
        #[cfg(unix)]
        {
            use std::fs::Permissions;
            use std::os::unix::fs::PermissionsExt;

            let permissions = Permissions::from_mode(0o755);
            tokio::fs::set_permissions(&target_path, permissions)
                .await
                .map_err(AddBinaryError::MakeExecutable)?;
        }

        Ok(())
    }

    pub async fn remove_binary(&self, name: impl AsRef<str>) -> Result<(), RemoveBinaryError> {
        let binary_path = self.exec_dir.join(name.as_ref());

        tokio::fs::remove_file(&binary_path).await.or_else(|e| {
            if e.kind() == io::ErrorKind::NotFound {
                Ok(())
            } else {
                Err(RemoveBinaryError(e))
            }
        })
    }

    pub async fn installed_modules(
        &self,
    ) -> Result<Vec<InstalledModuleManifest>, InstalledModulesError> {
        let installed_dir = &self.install_dir;

        let mut modules = Vec::new();

        if self.options.search_legacy_path || self.options.auto_migrate_legacy_path {
            if let Some(modules_dir) = &self.legacy_modules_dir {
                if let Ok(mut read_dir) = tokio::fs::read_dir(modules_dir).await {
                    while let Ok(Some(entry)) = read_dir.next_entry().await {
                        let path = entry.path();
                        if !tokio::fs::metadata(&path)
                            .await
                            .map(|md| md.is_file())
                            .unwrap_or(false)
                        {
                            continue;
                        }

                        let Some(file_name) = path.file_name().and_then(|n| n.to_str()) else {
                            continue;
                        };

                        if let Ok(manifest) = read_manifest(&path).await {
                            if self.options.auto_migrate_legacy_path {
                                tracing::debug!(
                                    ?path,
                                    "found a legacy manifest file, migrating..."
                                );

                                let dst = installed_dir.join(file_name);

                                tokio::fs::rename(&path, &dst)
                                .await
                                .inspect_err(|e| {
                                    tracing::debug!(from = ?path, to = ?dst, "failed to migrate legacy manifest file: {e}")
                                })
                                .ok();
                            } else {
                                modules.push(manifest);
                            }
                        }
                    }
                }
            }
        }

        let mut read_dir = tokio::fs::read_dir(&installed_dir)
            .await
            .map_err(|e| InstalledModulesError::DirIo(installed_dir.clone(), e))?;

        while let Some(entry) = read_dir
            .next_entry()
            .await
            .map_err(|e| InstalledModulesError::DirIo(installed_dir.clone(), e))?
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
                .map_err(|e| InstalledModulesError::ReadManifestError(path, e))?;

            modules.push(manifest)
        }

        Ok(modules)
    }

    pub async fn enabled_modules(
        &self,
    ) -> Result<Vec<InstalledModuleManifest>, EnabledModulesError> {
        let enabled_dir = &self.enable_dir;

        let mut read_dir = tokio::fs::read_dir(&enabled_dir)
            .await
            .map_err(|e| EnabledModulesError::DirIo(enabled_dir.clone(), e))?;

        let mut modules = Vec::new();

        while let Some(entry) = read_dir
            .next_entry()
            .await
            .map_err(|e| EnabledModulesError::DirIo(enabled_dir.clone(), e))?
        {
            let path = entry.path();

            if !tokio::fs::symlink_metadata(&path)
                .await
                .map(|md| md.is_symlink())
                .unwrap_or(false)
            {
                continue;
            }

            let manifest_path = tokio::fs::read_link(&path)
                .await
                .map_err(|e| EnabledModulesError::LinkIo(path.clone(), e))?;

            let manifest_path = if manifest_path.is_absolute() {
                manifest_path
            } else {
                enabled_dir.join(&manifest_path)
            };

            let manifest = read_manifest(&manifest_path)
                .await
                .map_err(|e| EnabledModulesError::ReadManifestError(path, e))?;

            modules.push(manifest)
        }

        Ok(modules)
    }

    pub async fn is_module_installed(
        &self,
        module_name: impl AsRef<str>,
    ) -> Result<bool, IsModuleInstalledError> {
        self.find_manifest_file(module_name)
            .await
            .map(|path| path.is_some())
            .map_err(Into::into)
    }

    pub async fn is_module_enabled(
        &self,
        module_name: impl AsRef<str>,
    ) -> Result<bool, IsModuleEnabledError> {
        let path = self.enable_dir.join(module_name.as_ref());

        tokio::fs::symlink_metadata(&path)
            .await
            .map(|md| md.is_symlink())
            .or_else(|e| {
                if e.kind() == io::ErrorKind::NotFound {
                    Ok(false)
                } else {
                    Err(e.into())
                }
            })
    }

    pub async fn enable_module(&self, module_name: impl AsRef<str>) -> Result<(), EnableError> {
        let target_path = self
            .find_manifest_file(&module_name)
            .await?
            .ok_or(EnableError::NotInstalled)?;

        let target_path = if self
            .install_dir
            .parent()
            .zip(self.enable_dir.parent())
            .is_some_and(|(a, b)| a == b)
        {
            // This scope only runs if install_dir and enable_dir share the same parent directory.

            if target_path.starts_with(&self.install_dir) {
                // manifest is in installed directory: ../installed/manifest.json
                std::path::PathBuf::from("..")
                    .join(self.install_dir.file_name().unwrap())
                    .join(target_path.file_name().unwrap())
            } else {
                // manifest is in legacy location: ../manifest.yaml
                std::path::PathBuf::from("..").join(target_path.file_name().unwrap())
            }
        } else {
            target_path
        };

        let src_path = self.enable_dir.join(module_name.as_ref());

        #[cfg(unix)]
        let fut = tokio::fs::symlink(&target_path, &src_path);

        #[cfg(windows)]
        let fut = tokio::fs::symlink_file(&target_path, &src_path);

        match fut.await {
            Ok(_) => Ok(()),
            Err(err) if err.kind() == io::ErrorKind::AlreadyExists => {
                // disable and retry enabling one more time
                let _ = self.disable_module(module_name).await;

                #[cfg(unix)]
                let fut = tokio::fs::symlink(&target_path, &src_path);

                #[cfg(windows)]
                let fut = tokio::fs::symlink_file(&target_path, &src_path);

                fut.await.map_err(EnableError::Io)
            },
            Err(err) => Err(EnableError::Io(err)),
        }
    }

    pub async fn disable_module(&self, module_name: impl AsRef<str>) -> Result<(), DisableError> {
        let path = self.enable_dir.join(module_name.as_ref());

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
    ) -> Result<Option<PathBuf>, FindManifestError> {
        let install_dir = &self.install_dir;

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
                    return Err(FindManifestError(path, err));
                },
                _ => continue,
            }
        }

        if !self.options.search_legacy_path {
            return Ok(None);
        }

        let legacy_dir = install_dir
            .parent()
            .expect("should never panic, self.install_dir() has >=2 path segments");
        for file in &files {
            let path = legacy_dir.join(file);
            match tokio::fs::try_exists(&path).await {
                Ok(exists) if exists => {
                    if self.options.auto_migrate_legacy_path {
                        tracing::debug!(?path, "found a legacy manifest file, migrating...");
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
                            .then_some(dst)
                            .or(Some(path)));
                    } else {
                        return Ok(Some(path));
                    }
                },
                Err(err) if err.kind() != io::ErrorKind::NotFound => {
                    return Err(FindManifestError(path, err));
                },
                _ => continue,
            }
        }

        Ok(None)
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

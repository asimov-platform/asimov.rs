// This is free and unencumbered software released into the public domain.

use alloc::{string::String, vec::Vec};
use asimov_module::InstalledModuleManifest;
use std::path::{Path, PathBuf};
use tokio::io;

pub mod error;
use error::*;

pub const MANIFEST_FILE_NAME: &str = "manifest.json";
pub const README_FILE_PATH: &str = "doc/README.md";
pub const BIN_DIR_NAME: &str = "bin";

const MANIFEST_FILE_NAMES: [&str; 3] = [MANIFEST_FILE_NAME, "manifest.yaml", "manifest.yml"];

#[derive(Clone, Debug, bon::Builder)]
pub struct Options {
    /// Controls whether to search for module manifests from a legacy location.
    /// The legacy (previous) location by default is `~/.asimov/modules/*.yaml`.
    #[builder(default = true)]
    pub search_legacy_path: bool,

    /// Controls whether to automatically move module manifests from a legacy location.
    /// The legacy (previous) location by default is `~/.asimov/modules/*.yaml`.
    /// The new and current location by default is `~/.asimov/modules/installed/<name>/manifest.json`.
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

    pub fn install_dir(&self) -> &Path {
        &self.install_dir
    }

    pub fn module_dir(&self, module_name: impl AsRef<str>) -> PathBuf {
        self.install_dir.join(module_name.as_ref())
    }

    pub async fn add_module(
        &self,
        module_name: impl AsRef<str>,
        dir: impl AsRef<Path>,
    ) -> Result<(), AddModuleError> {
        let module_name = module_name.as_ref();

        if self.is_module_installed(module_name).await.unwrap_or(false) {
            return Err(AddModuleError::AlreadyInstalled);
        }

        let module_dir = self.module_dir(module_name);

        tokio::fs::rename(dir.as_ref(), &module_dir)
            .await
            .map_err(|e| AddModuleError::Install(dir.as_ref().into(), module_dir.clone(), e))?;

        let bin_dir = module_dir.join(BIN_DIR_NAME);

        let mut entries = match tokio::fs::read_dir(&bin_dir).await {
            Ok(entries) => entries,
            Err(err) if err.kind() == io::ErrorKind::NotFound => return Ok(()),
            Err(err) => return Err(AddModuleError::ReadBinDir(bin_dir, err)),
        };

        while let Some(entry) = entries
            .next_entry()
            .await
            .map_err(|e| AddModuleError::ReadBinDir(bin_dir.clone(), e))?
        {
            let program_name = entry.file_name();
            let Some(program_name) = program_name.to_str() else {
                continue;
            };

            self.add_binary(program_name, &entry.path())
                .await
                .map_err(|e| AddModuleError::AddBinary(program_name.into(), e))?;
        }

        Ok(())
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

    pub async fn read_readme(
        &self,
        module_name: impl AsRef<str>,
    ) -> Result<Option<String>, ReadReadmeError> {
        let readme_path = self.module_dir(module_name).join(README_FILE_PATH);

        match tokio::fs::read_to_string(&readme_path).await {
            Ok(content) => Ok(Some(content)),
            Err(err) if err.kind() == io::ErrorKind::NotFound => Ok(None),
            Err(err) => Err(ReadReadmeError(readme_path, err)),
        }
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

    pub async fn remove_module(
        &self,
        module_name: impl AsRef<str>,
    ) -> Result<(), RemoveModuleError> {
        let manifest_path = self
            .find_manifest_file(&module_name)
            .await?
            .ok_or(RemoveModuleError::NotInstalled)?;

        let module_dir = self.module_dir(&module_name);

        if manifest_path.starts_with(&module_dir) {
            tokio::fs::remove_dir_all(&module_dir)
                .await
                .map_err(|e| RemoveModuleError::RemoveModuleDir(module_dir, e))
        } else {
            tokio::fs::remove_file(&manifest_path)
                .await
                .map_err(|e| RemoveModuleError::RemoveManifest(manifest_path, e))
        }
    }

    async fn add_binary(&self, program_name: &str, binary_path: &Path) -> io::Result<()> {
        let target_path = match self
            .exec_dir
            .parent()
            .and_then(|parent| binary_path.strip_prefix(parent).ok())
        {
            Some(suffix) => PathBuf::from("..").join(suffix),
            None => binary_path.into(),
        };

        let link_path = self.exec_dir.join(program_name);
        let _ = self.remove_binary(program_name).await;

        create_symlink(&target_path, &link_path, false).await
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

        if (self.options.search_legacy_path || self.options.auto_migrate_legacy_path)
            && let Some(modules_dir) = &self.legacy_modules_dir
            && let Ok(mut read_dir) = tokio::fs::read_dir(modules_dir).await
        {
            while let Ok(Some(entry)) = read_dir.next_entry().await {
                let path = entry.path();
                if !tokio::fs::metadata(&path)
                    .await
                    .map(|md| md.is_file())
                    .unwrap_or(false)
                {
                    continue;
                }

                let Some(module_name) = manifest_file_module_name(&path) else {
                    continue;
                };

                if let Ok(manifest) = read_manifest(&path).await {
                    if self.options.auto_migrate_legacy_path {
                        tracing::debug!(?path, "found a legacy manifest file, migrating...");

                        self.migrate_legacy_manifest(module_name, &path).await.ok();
                    } else {
                        modules.push(manifest);
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

            let Ok(metadata) = tokio::fs::metadata(&path).await else {
                continue;
            };

            if metadata.is_dir() {
                let Some(manifest_path) = find_manifest_in_dir(&path)
                    .await
                    .map_err(|e| InstalledModulesError::DirIo(path.clone(), e))?
                else {
                    continue;
                };

                let manifest = read_manifest(&manifest_path)
                    .await
                    .map_err(|e| InstalledModulesError::ReadManifestError(manifest_path, e))?;

                modules.push(manifest)
            } else if metadata.is_file() {
                let Some(module_name) = manifest_file_module_name(&path) else {
                    continue;
                };

                let manifest = read_manifest(&path)
                    .await
                    .map_err(|e| InstalledModulesError::ReadManifestError(path.clone(), e))?;

                if self.options.auto_migrate_legacy_path {
                    tracing::debug!(?path, "found a legacy manifest file, migrating...");

                    self.migrate_legacy_manifest(module_name, &path).await.ok();
                }

                modules.push(manifest)
            }
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

            let manifest_path = if tokio::fs::metadata(&manifest_path)
                .await
                .map(|md| md.is_dir())
                .unwrap_or(false)
            {
                match find_manifest_in_dir(&manifest_path).await {
                    Ok(Some(manifest_path)) => manifest_path,
                    Ok(None) => continue,
                    Err(e) => return Err(EnabledModulesError::DirIo(manifest_path, e)),
                }
            } else {
                manifest_path
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
        let manifest_path = self
            .find_manifest_file(&module_name)
            .await?
            .ok_or(EnableError::NotInstalled)?;

        self.link_module(module_name.as_ref(), &manifest_path)
            .await
            .map_err(EnableError::Io)
    }

    pub async fn disable_module(&self, module_name: impl AsRef<str>) -> Result<(), DisableError> {
        let path = self.enable_dir.join(module_name.as_ref());

        let result = match tokio::fs::remove_file(&path).await {
            Err(err) if err.kind() != io::ErrorKind::NotFound => {
                tokio::fs::remove_dir(&path).await.or(Err(err))
            },
            result => result,
        };

        result
            .or_else(|err| {
                if err.kind() == io::ErrorKind::NotFound {
                    Ok(())
                } else {
                    Err(err)
                }
            })
            .map_err(Into::into)
    }

    async fn link_module(&self, module_name: &str, manifest_path: &Path) -> io::Result<()> {
        let module_dir = self.module_dir(module_name);

        let target_path = if manifest_path.starts_with(&module_dir) {
            module_dir
        } else {
            manifest_path.into()
        };
        let target_is_dir = tokio::fs::metadata(&target_path)
            .await
            .map(|md| md.is_dir())
            .unwrap_or(false);

        let target_path = if self
            .install_dir
            .parent()
            .zip(self.enable_dir.parent())
            .is_some_and(|(a, b)| a == b)
        {
            // This scope only runs if install_dir and enable_dir share the same parent directory.

            if target_path.starts_with(&self.install_dir) {
                // manifest is in installed directory: ../installed/<name>
                PathBuf::from("..")
                    .join(self.install_dir.file_name().unwrap())
                    .join(target_path.file_name().unwrap())
            } else {
                // manifest is in legacy location: ../<name>.yaml
                PathBuf::from("..").join(target_path.file_name().unwrap())
            }
        } else {
            target_path
        };

        let src_path = self.enable_dir.join(module_name);

        match create_symlink(&target_path, &src_path, target_is_dir).await {
            Err(err) if err.kind() == io::ErrorKind::AlreadyExists => {
                // disable and retry enabling one more time
                let _ = self.disable_module(module_name).await;

                create_symlink(&target_path, &src_path, target_is_dir).await
            },
            result => result,
        }
    }

    async fn find_manifest_file(
        &self,
        module_name: impl AsRef<str>,
    ) -> Result<Option<PathBuf>, FindManifestError> {
        use alloc::{format, vec};

        let module_name = module_name.as_ref();
        let module_dir = self.module_dir(module_name);

        if let Some(path) = find_manifest_in_dir(&module_dir)
            .await
            .map_err(|err| FindManifestError(module_dir, err))?
        {
            return Ok(Some(path));
        }

        if !self.options.search_legacy_path {
            return Ok(None);
        }

        let files = [
            format!("{module_name}.json"),
            format!("{module_name}.yaml"),
            format!("{module_name}.yml"),
        ];

        let mut legacy_dirs = vec![self.install_dir.clone()];
        if let Some(parent) = self.install_dir.parent() {
            legacy_dirs.push(parent.into());
        }

        for dir in &legacy_dirs {
            for file in &files {
                let path = dir.join(file);
                match tokio::fs::try_exists(&path).await {
                    Ok(exists) if exists => {
                        if !self.options.auto_migrate_legacy_path {
                            return Ok(Some(path));
                        }

                        tracing::debug!(?path, "found a legacy manifest file, migrating...");

                        return Ok(Some(
                            self.migrate_legacy_manifest(module_name, &path)
                                .await
                                .unwrap_or(path),
                        ));
                    },
                    Err(err) if err.kind() != io::ErrorKind::NotFound => {
                        return Err(FindManifestError(path, err));
                    },
                    _ => continue,
                }
            }
        }

        Ok(None)
    }

    async fn migrate_legacy_manifest(&self, module_name: &str, path: &Path) -> io::Result<PathBuf> {
        let module_dir = self.module_dir(module_name);

        tokio::fs::create_dir_all(&module_dir).await?;

        let extension = path.extension().unwrap_or_else(|| "json".as_ref());
        let dst = module_dir.join("manifest").with_extension(extension);

        tokio::fs::rename(path, &dst).await.inspect_err(|err| {
            tracing::debug!(
                from = ?path,
                to = ?dst,
                ?err,
                "tried to move module manifest from legacy path but failed"
            )
        })?;

        if self.is_module_enabled(module_name).await.unwrap_or(false) {
            let _ = self.disable_module(module_name).await;
            let _ = self.link_module(module_name, &dst).await;
        }

        Ok(dst)
    }
}

#[cfg(unix)]
async fn create_symlink(target: &Path, link: &Path, _target_is_dir: bool) -> io::Result<()> {
    tokio::fs::symlink(target, link).await
}

#[cfg(windows)]
async fn create_symlink(target: &Path, link: &Path, target_is_dir: bool) -> io::Result<()> {
    if target_is_dir {
        tokio::fs::symlink_dir(target, link).await
    } else {
        tokio::fs::symlink_file(target, link).await
    }
}

async fn find_manifest_in_dir(module_dir: &Path) -> io::Result<Option<PathBuf>> {
    for file in MANIFEST_FILE_NAMES {
        let path = module_dir.join(file);
        match tokio::fs::try_exists(&path).await {
            Ok(exists) if exists => return Ok(Some(path)),
            Err(err) if err.kind() != io::ErrorKind::NotFound => return Err(err),
            _ => continue,
        }
    }

    Ok(None)
}

fn manifest_file_module_name(path: &Path) -> Option<&str> {
    match path.extension().and_then(|ext| ext.to_str()) {
        Some("json") | Some("yaml") | Some("yml") => {
            path.file_stem().and_then(|name| name.to_str())
        },
        _ => None,
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

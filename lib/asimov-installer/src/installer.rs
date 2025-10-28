// This is free and unencumbered software released into the public domain.

use asimov_module::{InstalledModuleManifest, ModuleManifest, tracing};
use std::{path::Path, string::String};

pub mod error;
use error::*;

use asimov_registry::Registry;

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
            .connect_timeout(std::time::Duration::from_secs(10))
            .read_timeout(std::time::Duration::from_secs(30))
            .build()
            .expect("Failed to build HTTP client");
        let registry = Registry::default();
        Self::new(client, registry)
    }
}

#[derive(Clone, Debug, Default, bon::Builder)]
#[builder(on(String, into))]
pub struct InstallOptions {
    pub version: Option<String>,
    pub model_size: Option<String>,
}

impl Installer {
    pub fn new(client: reqwest::Client, registry: Registry) -> Self {
        Self { client, registry }
    }

    /// ```rust,no_run
    /// # use asimov_installer::{Installer, InstallOptions};
    /// let i = Installer::default();
    /// i.install_module("foobar", &InstallOptions::default());
    /// ```
    pub async fn install_module(
        &self,
        module: impl AsRef<str> + 'static,
        options: &InstallOptions,
    ) -> Result<(), InstallError> {
        let temp_dir = tempfile::Builder::new()
            .prefix("asimov-module-installer")
            .tempdir()
            .map_err(InstallError::CreateTempDir)?;

        let (manifest, version) = self
            .preinstall(module.as_ref(), options, temp_dir.path())
            .await?;

        self.finish_install(version.as_ref(), manifest, temp_dir.path())
            .await?;

        Ok(())
    }

    pub async fn fetch_latest_release(
        &self,
        module_name: impl AsRef<str>,
    ) -> Result<String, FetchError> {
        github::fetch_latest_release(&self.client, module_name).await
    }

    /// ```rust,no_run
    /// # use asimov_installer::{Installer, InstallOptions};
    /// let i = Installer::default();
    /// i.upgrade_module("foobar", &InstallOptions::default());
    /// ```
    pub async fn upgrade_module(
        &self,
        module: impl AsRef<str> + 'static,
        options: &InstallOptions,
    ) -> Result<(), UpgradeError> {
        let module_name = module.as_ref();

        let version = if let Some(ref want_version) = options.version {
            want_version.clone()
        } else {
            self.fetch_latest_release(module_name).await?
        };

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

        let (manifest, version) = self
            .preinstall(module_name, options, temp_dir.path())
            .await?;

        // now ok to uninstall old version
        self.uninstall_module(module_name).await?;

        self.finish_install(&version, manifest, temp_dir.path())
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
        let manifest = self.registry.read_manifest(&module_name).await?;

        self.registry.disable_module(&module_name).await?;

        for program in &manifest.manifest.provides.programs {
            self.registry
                .remove_binary(program)
                .await
                .map_err(|e| UninstallError::RemoveBinary(program.into(), e))?;
        }

        self.registry.remove_manifest(&module_name).await?;

        Ok(())
    }

    async fn preinstall(
        &self,
        module_name: &str,
        options: &InstallOptions,
        temp_dir: &Path,
    ) -> Result<(ModuleManifest, String), PreinstallError> {
        let platform = platform::detect_platform();

        let version = if let Some(ref want_version) = options.version {
            want_version.clone()
        } else {
            github::fetch_latest_release(&self.client, module_name)
                .await
                .map_err(PreinstallError::FetchRelease)?
        };

        let release = github::fetch_release(&self.client, module_name, &version)
            .await
            .map_err(PreinstallError::FetchRelease)?;

        let Some(asset) = github::find_matching_asset(&release.assets, module_name, &platform)
        else {
            return Err(PreinstallError::NotAvailable(platform));
        };

        let manifest = github::fetch_module_manifest(&self.client, module_name, &version)
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

        if let Some(ref requires) = manifest.requires {
            for (name, model) in &requires.models {
                let Some(repo) = name.strip_prefix("hf:") else {
                    tracing::debug!(
                        ?name,
                        "unexpected format for required model, only `hf:<user>/<repo>` is supported"
                    );
                    continue;
                };

                use asimov_module::RequiredModel;
                let filename = match (model, &options.model_size) {
                    (RequiredModel::Url(url), None | Some(_)) => url,
                    (RequiredModel::Choices(choices), None) => {
                        let Some((_, model)) = choices.first() else {
                            // malformed manifest?
                            tracing::warn!(
                                ?module_name,
                                "manifest defines required models with no choices"
                            );
                            continue;
                        };
                        model
                    },
                    (RequiredModel::Choices(choices), Some(want_model)) => {
                        &choices
                            .iter()
                            .find(|(name, _)| *name == *want_model)
                            .ok_or_else(|| PreinstallError::NoSuchModel(want_model.clone()))?
                            .1
                    },
                };

                asimov_huggingface::ensure_file(repo, filename)?;
            }
        }

        Ok((manifest, version))
    }

    async fn finish_install(
        &self,
        version: &str,
        manifest: ModuleManifest,
        temp_dir: &Path,
    ) -> Result<(), FinishInstallError> {
        let extract_dir = temp_dir.join("extract");

        for program in &manifest.provides.programs {
            let src = extract_dir.join(program);
            self.registry.add_binary(program, &src).await?;
        }

        let installed_manifest = InstalledModuleManifest {
            version: Some(version.into()),
            manifest,
        };

        self.registry.add_manifest(installed_manifest).await?;

        Ok(())
    }
}

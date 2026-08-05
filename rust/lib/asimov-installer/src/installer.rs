// This is free and unencumbered software released into the public domain.

use asimov_module::{InstalledModuleManifest, ModuleManifest, tracing};
use std::{boxed::Box, path::Path, string::String};

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

#[derive(Clone, Debug)]
struct Preinstalled {
    manifest: ModuleManifest,
    version: String,
    readme: Option<String>,
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

        let preinstalled = self
            .preinstall(module.as_ref(), options, temp_dir.path())
            .await?;

        self.finish_install(preinstalled, temp_dir.path()).await?;

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

        let preinstalled = self
            .preinstall(module_name, options, temp_dir.path())
            .await?;

        // now ok to uninstall old version
        self.uninstall_module(module_name).await?;

        self.finish_install(preinstalled, temp_dir.path()).await?;

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

        self.registry.remove_module(&module_name).await?;

        Ok(())
    }

    async fn preinstall(
        &self,
        module_name: &str,
        options: &InstallOptions,
        temp_dir: &Path,
    ) -> Result<Preinstalled, PreinstallError> {
        let platform = platform::detect_platform();

        let version = if let Some(ref want_version) = options.version {
            want_version.clone()
        } else {
            github::fetch_latest_release(&self.client, module_name)
                .await
                .map_err(PreinstallError::FetchRelease)?
        };

        let manifest = github::fetch_module_manifest(&self.client, module_name, &version)
            .await
            .map_err(PreinstallError::FetchManifest)?;

        let (asset_url, download_path) = github::download_matching_asset(
            &self.client,
            module_name,
            &version,
            &platform,
            temp_dir,
        )
        .await?;

        // pass the model_size option to dependencies
        let options = InstallOptions::builder()
            .maybe_model_size(options.model_size.clone())
            .build();
        let subdeps = &manifest.requires.modules;
        for module in subdeps {
            if self
                .registry
                .is_module_installed(&module)
                .await
                .unwrap_or(false)
            {
                continue;
            }
            Box::pin(self.install_module(module.clone(), &options))
                .await
                .map_err(|e| PreinstallError::Dependency(module.clone(), Box::new(e)))?;
        }

        match github::fetch_checksum(&self.client, &asset_url).await {
            Ok(None) => {},
            Ok(Some(checksum)) => {
                github::verify_checksum(&download_path, &checksum).await?;
            },
            Err(err) => Err(err)?,
        }

        let extract_dir = temp_dir.join("extract");
        tokio::fs::create_dir(&extract_dir)
            .await
            .map_err(PreinstallError::CreateExtractDir)?;

        github::extract_files(&download_path, &extract_dir)
            .await
            .map_err(PreinstallError::Extract)?;

        for (name, model) in &manifest.requires.models {
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
                    if choices
                        .iter()
                        .any(|(_, url)| asimov_huggingface::file_exists(repo, url).is_some())
                    {
                        // user didn't specify a model size/version to install
                        // and one of the choices is already installed
                        continue;
                    }
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

        let readme = match find_readme(&extract_dir).await {
            Some(readme) => Some(readme),
            None => github::fetch_readme(&self.client, module_name, &version).await,
        };

        Ok(Preinstalled {
            manifest,
            version,
            readme,
        })
    }

    async fn finish_install(
        &self,
        preinstalled: Preinstalled,
        temp_dir: &Path,
    ) -> Result<(), FinishInstallError> {
        let Preinstalled {
            manifest,
            version,
            readme,
        } = preinstalled;

        let extract_dir = temp_dir.join("extract");

        for program in &manifest.provides.programs {
            let src = extract_dir.join(program);

            // On Windows add the .exe extension to the binary name:
            #[cfg(windows)]
            let src = src.with_extension("exe");

            self.registry
                .add_binary(&manifest.name, program, &src)
                .await?;
        }

        let module_name = manifest.name.clone();

        let installed_manifest = InstalledModuleManifest {
            version: Some(version),
            manifest,
        };

        self.registry.add_manifest(installed_manifest).await?;

        if let Some(readme) = readme {
            self.registry.add_readme(&module_name, readme).await?;
        }

        Ok(())
    }
}

async fn find_readme(extract_dir: &Path) -> Option<String> {
    let mut entries = tokio::fs::read_dir(extract_dir).await.ok()?;

    while let Ok(Some(entry)) = entries.next_entry().await {
        let file_name = entry.file_name();
        let Some(file_name) = file_name.to_str() else {
            continue;
        };

        if !file_name.to_ascii_uppercase().starts_with("README") {
            continue;
        }

        if !entry
            .file_type()
            .await
            .map(|file_type| file_type.is_file())
            .unwrap_or(false)
        {
            continue;
        }

        return tokio::fs::read_to_string(entry.path()).await.ok();
    }

    None
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn find_readme_in_archive() {
        let dir = tempfile::tempdir().unwrap();

        assert_eq!(find_readme(dir.path()).await, None);

        std::fs::create_dir(dir.path().join("README")).unwrap();
        assert_eq!(find_readme(dir.path()).await, None);

        std::fs::write(dir.path().join("readme.md"), "# Hello").unwrap();
        assert_eq!(find_readme(dir.path()).await.as_deref(), Some("# Hello"));
    }
}

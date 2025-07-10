// This is free and unencumbered software released into the public domain.

use alloc::{string::String, vec::Vec};

#[derive(Clone, Debug, Default)]
#[cfg_attr(feature = "serde", derive(serde::Deserialize, serde::Serialize))]
pub struct ModuleManifest {
    pub name: String,
    pub label: String,
    pub summary: String,
    pub links: Vec<String>,

    #[cfg_attr(
        feature = "serde",
        serde(default, skip_serializing_if = "Provides::is_empty")
    )]
    pub provides: Provides,

    #[cfg_attr(
        feature = "serde",
        serde(default, skip_serializing_if = "Handles::is_empty")
    )]
    pub handles: Handles,

    #[cfg_attr(
        feature = "serde",
        serde(alias = "configuration", skip_serializing_if = "Option::is_none")
    )]
    pub config: Option<Configuration>,
}

#[cfg(feature = "std")]
#[derive(Debug, thiserror::Error)]
pub enum ReadVarError {
    #[error("variable named `{0}` not found in module manifest")]
    UnknownVar(String),

    #[error("a value for variable `{0}` was not configured")]
    UnconfiguredVar(String),

    #[error("failed to read variable `{name}`: {source}")]
    Io {
        name: String,
        #[source]
        source: std::io::Error,
    },
}

impl ModuleManifest {
    #[cfg(feature = "std")]
    pub fn read_manifest(module_name: &str) -> std::io::Result<Self> {
        let path = asimov_env::paths::asimov_root()
            .join("modules")
            .join(std::format!("{module_name}.yaml"));
        let content = std::fs::read(&path)?;
        serde_yml::from_slice(&content).map_err(std::io::Error::other)
    }

    #[cfg(feature = "std")]
    pub fn read_variables(
        &self,
        profile: Option<&str>,
    ) -> Result<std::collections::BTreeMap<String, String>, ReadVarError> {
        self.config
            .as_ref()
            .map(|c| c.variables.as_slice())
            .unwrap_or_default()
            .iter()
            .map(|var| Ok((var.name.clone(), self.variable(&var.name, profile)?)))
            .collect()
    }

    #[cfg(feature = "std")]
    pub fn variable(&self, key: &str, profile: Option<&str>) -> Result<String, ReadVarError> {
        let Some(var) = self
            .config
            .as_ref()
            .and_then(|conf| conf.variables.iter().find(|var| var.name == key))
        else {
            return Err(ReadVarError::UnknownVar(key.into()));
        };

        if let Some(value) = var
            .environment
            .as_deref()
            .and_then(|env_name| std::env::var(env_name).ok())
        {
            return Ok(value);
        }

        let profile = profile.unwrap_or("default");
        let path = asimov_env::paths::asimov_root()
            .join("configs")
            .join(profile)
            .join(&self.name)
            .join(key);

        std::fs::read_to_string(&path).or_else(|err| {
            if err.kind() == std::io::ErrorKind::NotFound {
                var.default_value
                    .clone()
                    .ok_or_else(|| ReadVarError::UnconfiguredVar(key.into()))
            } else {
                Err(ReadVarError::Io {
                    name: key.into(),
                    source: err,
                })
            }
        })
    }
}

#[derive(Clone, Debug, Default)]
#[cfg_attr(feature = "serde", derive(serde::Deserialize, serde::Serialize))]
pub struct Provides {
    pub programs: Vec<String>,
}

impl Provides {
    pub fn is_empty(&self) -> bool {
        self.programs.is_empty()
    }
}

#[derive(Clone, Debug, Default)]
#[cfg_attr(feature = "serde", derive(serde::Deserialize, serde::Serialize))]
pub struct Handles {
    #[cfg_attr(
        feature = "serde",
        serde(default, skip_serializing_if = "Vec::is_empty")
    )]
    pub url_protocols: Vec<String>,

    #[cfg_attr(
        feature = "serde",
        serde(default, skip_serializing_if = "Vec::is_empty")
    )]
    pub url_prefixes: Vec<String>,

    #[cfg_attr(
        feature = "serde",
        serde(default, skip_serializing_if = "Vec::is_empty")
    )]
    pub url_patterns: Vec<String>,

    #[cfg_attr(
        feature = "serde",
        serde(default, skip_serializing_if = "Vec::is_empty")
    )]
    pub file_extensions: Vec<String>,

    #[cfg_attr(
        feature = "serde",
        serde(default, skip_serializing_if = "Vec::is_empty")
    )]
    pub content_types: Vec<String>,
}

impl Handles {
    pub fn is_empty(&self) -> bool {
        self.url_protocols.is_empty()
            && self.url_prefixes.is_empty()
            && self.url_patterns.is_empty()
            && self.file_extensions.is_empty()
            && self.content_types.is_empty()
    }
}

#[derive(Clone, Debug, Default)]
#[cfg_attr(feature = "serde", derive(serde::Deserialize, serde::Serialize))]
pub struct Configuration {
    #[cfg_attr(
        feature = "serde",
        serde(default, skip_serializing_if = "Vec::is_empty")
    )]
    pub variables: Vec<ConfigurationVariable>,
}

#[derive(Clone, Debug, Default)]
#[cfg_attr(feature = "serde", derive(serde::Deserialize, serde::Serialize))]
pub struct ConfigurationVariable {
    /// The name of the variable. Configured variables are by default saved in
    /// `~/.asimov/configs/$profile/$module/$name`.
    pub name: String,

    /// Optional description to provide information about the variable.
    #[cfg_attr(
        feature = "serde",
        serde(default, alias = "desc", skip_serializing_if = "Option::is_none")
    )]
    pub description: Option<String>,

    /// Optional name of an environment variable to check for a value before checking for a
    /// configured or a default value.
    #[cfg_attr(
        feature = "serde",
        serde(default, alias = "env", skip_serializing_if = "Option::is_none")
    )]
    pub environment: Option<String>,

    /// Optional default value to use as a fallback. If a default value is present the user
    /// configuration of the value is not required.
    #[cfg_attr(
        feature = "serde",
        serde(default, alias = "default", skip_serializing_if = "Option::is_none")
    )]
    pub default_value: Option<String>,
}

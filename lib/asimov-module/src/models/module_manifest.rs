// This is free and unencumbered software released into the public domain.

use alloc::{collections::BTreeMap, string::String, vec::Vec};

#[derive(Clone, Debug, Default, PartialEq, Eq)]
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

    #[cfg_attr(feature = "serde", serde(skip_serializing_if = "Option::is_none"))]
    pub requires: Option<Requires>,
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
    #[cfg(all(feature = "std", feature = "serde"))]
    pub fn read_manifest(module_name: &str) -> std::io::Result<Self> {
        let directory = asimov_env::paths::asimov_root().join("modules");
        let search_paths = [
            ("installed", "json"),
            ("installed", "yaml"), // legacy, new installs are converted to JSON
            ("", "yaml"),          // legacy, new installs go to `installed/`
        ];

        for (sub_dir, ext) in search_paths {
            let file = std::path::PathBuf::from(sub_dir)
                .join(module_name)
                .with_extension(ext);

            match std::fs::read(directory.join(&file)) {
                Ok(content) if ext == "json" => {
                    return serde_json::from_slice(&content).map_err(std::io::Error::other);
                },
                Ok(content) if ext == "yaml" => {
                    return serde_yaml_ng::from_slice(&content).map_err(std::io::Error::other);
                },
                Ok(_) => unreachable!(),

                Err(err) if err.kind() == std::io::ErrorKind::NotFound => continue,
                Err(err) => return Err(err),
            }
        }

        Err(std::io::ErrorKind::NotFound.into())
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

#[derive(Clone, Debug, Default, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(serde::Deserialize, serde::Serialize))]
pub struct Provides {
    pub programs: Vec<String>,
}

impl Provides {
    pub fn is_empty(&self) -> bool {
        self.programs.is_empty()
    }
}

#[cfg(feature = "serde")]
fn empty_vec_if_null<'de, D, T>(deserializer: D) -> Result<Vec<T>, D::Error>
where
    D: serde::Deserializer<'de>,
    T: serde::Deserialize<'de>,
{
    use serde::Deserialize;
    Ok(Option::<Vec<T>>::deserialize(deserializer)?.unwrap_or_default())
}

#[derive(Clone, Debug, Default, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(serde::Deserialize, serde::Serialize))]
pub struct Handles {
    #[cfg_attr(
        feature = "serde",
        serde(
            default,
            deserialize_with = "empty_vec_if_null",
            skip_serializing_if = "Vec::is_empty"
        )
    )]
    pub url_protocols: Vec<String>,

    #[cfg_attr(
        feature = "serde",
        serde(
            default,
            deserialize_with = "empty_vec_if_null",
            skip_serializing_if = "Vec::is_empty"
        )
    )]
    pub url_prefixes: Vec<String>,

    #[cfg_attr(
        feature = "serde",
        serde(
            default,
            deserialize_with = "empty_vec_if_null",
            skip_serializing_if = "Vec::is_empty"
        )
    )]
    pub url_patterns: Vec<String>,

    #[cfg_attr(
        feature = "serde",
        serde(
            default,
            deserialize_with = "empty_vec_if_null",
            skip_serializing_if = "Vec::is_empty"
        )
    )]
    pub file_extensions: Vec<String>,

    #[cfg_attr(
        feature = "serde",
        serde(
            default,
            deserialize_with = "empty_vec_if_null",
            skip_serializing_if = "Vec::is_empty"
        )
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

#[derive(Clone, Debug, Default, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(serde::Deserialize, serde::Serialize))]
pub struct Configuration {
    #[cfg_attr(
        feature = "serde",
        serde(default, skip_serializing_if = "Vec::is_empty")
    )]
    pub variables: Vec<ConfigurationVariable>,
}

#[derive(Clone, Debug, Default, PartialEq, Eq)]
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

#[derive(Clone, Debug, Default, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(serde::Deserialize, serde::Serialize))]
pub struct Requires {
    /// List of modules that this module depends on.
    #[cfg_attr(
        feature = "serde",
        serde(
            default,
            deserialize_with = "empty_vec_if_null",
            skip_serializing_if = "Vec::is_empty"
        )
    )]
    pub modules: Vec<String>,

    #[cfg_attr(
        feature = "serde",
        serde(default, skip_serializing_if = "BTreeMap::is_empty")
    )]
    pub models: BTreeMap<String, RequiredModel>,
}

#[derive(Clone, Debug, PartialEq, Eq)]
#[cfg_attr(
    feature = "serde",
    derive(serde::Deserialize, serde::Serialize),
    serde(untagged)
)]
pub enum RequiredModel {
    /// Just a direct URL string:
    /// ```yaml
    /// hf:first/model: model_file.bin
    /// ```
    Url(String),

    /// Multiple variants:
    /// ```yaml
    /// hf:second/model:
    ///   small: model_small.bin
    ///   medium: model_medium.bin
    ///   large: model_large.bin
    /// ```
    #[cfg_attr(
        feature = "serde",
        serde(deserialize_with = "ordered::deserialize_ordered")
    )]
    Choices(Vec<(String, String)>),
}

#[cfg(feature = "serde")]
mod ordered {
    use super::*;
    use serde::{
        Deserializer,
        de::{MapAccess, Visitor},
    };
    use std::fmt;

    pub fn deserialize_ordered<'de, D>(deserializer: D) -> Result<Vec<(String, String)>, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct OrderedVisitor;

        impl<'de> Visitor<'de> for OrderedVisitor {
            type Value = Vec<(String, String)>;

            fn expecting(&self, f: &mut fmt::Formatter) -> fmt::Result {
                f.write_str("a map of string keys to string values (preserving order)")
            }

            fn visit_map<A>(self, mut access: A) -> Result<Self::Value, A::Error>
            where
                A: MapAccess<'de>,
            {
                let mut items = Vec::with_capacity(access.size_hint().unwrap_or(0));
                while let Some((k, v)) = access.next_entry::<String, String>()? {
                    items.push((k, v));
                }
                Ok(items)
            }
        }

        deserializer.deserialize_map(OrderedVisitor)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::*;

    #[test]
    fn test_deser() {
        let yaml = r#"
name: example
label: Example
summary: Example Module
links:
  - https://github.com/asimov-platform/asimov.rs/tree/master/lib/asimov-module

provides:
  programs:
    - asimov-example-module

handles:
  content_types:
    - content_type
  file_extensions:
    - file_extension
  url_patterns:
    - pattern
  url_prefixes:
    - prefix
  url_protocols:
    - protocol

config:
  variables:
    - name: api_key
      description: "api key to authorize requests"
      default_value: "foobar"
      environment: API_KEY

requires:
  modules:
    - other
  models:
    hf:first/model: first_url
    hf:second/model:
      small: small_url
      medium: medium_url
      large: large_url
"#;

        let dec: ModuleManifest = serde_yaml_ng::from_str(yaml).expect("deser should succeed");

        assert_eq!("example", dec.name);
        assert_eq!("Example", dec.label);
        assert_eq!("Example Module", dec.summary);

        assert_eq!(
            vec!["https://github.com/asimov-platform/asimov.rs/tree/master/lib/asimov-module"],
            dec.links
        );

        assert_eq!(1, dec.provides.programs.len());
        assert_eq!(
            "asimov-example-module",
            dec.provides.programs.first().unwrap()
        );

        assert_eq!(
            "content_type",
            dec.handles
                .content_types
                .first()
                .expect("should have content_types")
        );

        assert_eq!(
            "file_extension",
            dec.handles
                .file_extensions
                .first()
                .expect("should have file_extensions")
        );

        assert_eq!(
            "pattern",
            dec.handles
                .url_patterns
                .first()
                .expect("should have url_patterns")
        );

        assert_eq!(
            "prefix",
            dec.handles
                .url_prefixes
                .first()
                .expect("should have url_prefixes")
        );

        assert_eq!(
            "protocol",
            dec.handles
                .url_protocols
                .first()
                .expect("should have url_protocols")
        );

        assert_eq!(
            Some(&ConfigurationVariable {
                name: "api_key".into(),
                description: Some("api key to authorize requests".into()),
                environment: Some("API_KEY".into()),
                default_value: Some("foobar".into())
            }),
            dec.config.expect("should have config").variables.first()
        );

        let requires = dec.requires.expect("should have requires");

        assert_eq!(1, requires.modules.len());
        assert_eq!("other", requires.modules.first().unwrap());

        assert_eq!(2, requires.models.len());

        assert_eq!(
            RequiredModel::Url("first_url".into()),
            requires.models["hf:first/model"]
        );

        assert_eq!(
            RequiredModel::Choices(vec![
                ("small".into(), "small_url".into()),
                ("medium".into(), "medium_url".into()),
                ("large".into(), "large_url".into())
            ]),
            requires.models["hf:second/model"]
        );
    }
}

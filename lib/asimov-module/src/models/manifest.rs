// This is free and unencumbered software released into the public domain.

use alloc::{string::String, vec::Vec};

#[derive(Clone, Debug, Default)]
#[cfg_attr(feature = "serde", derive(serde::Deserialize, serde::Serialize))]
pub struct Manifest {
    pub name: String,
    pub label: String,
    pub summary: String,
    pub links: Vec<String>,
    #[cfg_attr(
        feature = "serde",
        serde(default, skip_serializing_if = "Flows::is_empty")
    )]
    pub provides: Flows,
    #[cfg_attr(
        feature = "serde",
        serde(default, skip_serializing_if = "Handles::is_empty")
    )]
    pub handles: Handles,
}

#[derive(Clone, Debug, Default)]
#[cfg_attr(feature = "serde", derive(serde::Deserialize, serde::Serialize))]
pub struct Flows {
    pub flows: Vec<String>,
}

impl Flows {
    pub fn is_empty(&self) -> bool {
        self.flows.is_empty()
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

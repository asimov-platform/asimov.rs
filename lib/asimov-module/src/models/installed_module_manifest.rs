// This is free and unencumbered software released into the public domain.

use alloc::string::String;

#[derive(Clone, Debug, Default)]
#[cfg_attr(feature = "serde", derive(serde::Deserialize, serde::Serialize))]
pub struct InstalledModuleManifest {
    pub version: Option<String>,

    #[cfg_attr(feature = "serde", serde(flatten))]
    pub manifest: super::ModuleManifest,
}

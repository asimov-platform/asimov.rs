// This is free and unencumbered software released into the public domain.

use std::{io, path::PathBuf, string::String};

use thiserror::Error;

#[derive(Debug, Error)]
pub enum ReadError {
    #[error("failed to read directory for installed modules `{path}`: {source}")]
    InstallDirIo {
        path: PathBuf,
        #[source]
        source: io::Error,
    },
    #[error("failed to read directory for enabled modules `{path}`: {source}")]
    EnableDirIo {
        path: PathBuf,
        #[source]
        source: io::Error,
    },
    #[error("failed to read symlink for enabled module at `{path}`: {source}")]
    EnabledLinkIo {
        path: PathBuf,
        #[source]
        source: io::Error,
    },
    #[error(transparent)]
    ReadManifestError(#[from] ReadManifestError),
}

#[derive(Debug, Error)]
pub enum ReadManifestError {
    #[error("failed to read installed module manifest at `{path}`: {source}")]
    InstalledManifestIo {
        path: PathBuf,
        #[source]
        source: io::Error,
    },
    #[error("failed to deserialize module manifest at `{path}`: {source}")]
    ManifestDeserialize {
        path: PathBuf,
        #[source]
        source: DeserializeError,
    },
    #[error("unknown manifest format at `{path}`: {}", extension.as_deref().unwrap_or("no file extension"))]
    UnknownManifestFormat {
        path: PathBuf,
        extension: Option<String>,
    },
}

#[derive(Debug, Error)]
pub enum DeserializeError {
    #[error("JSON deserialization failed: {0}")]
    Json(#[from] serde_json::Error),
    #[error("YAML deserialization failed: {0}")]
    Yaml(#[from] serde_yml::Error),
}

#[derive(Debug, Error)]
pub enum FetchError {}

#[derive(Debug, Error)]
pub enum ReadModuleVersionError {
    #[error(transparent)]
    ReadError(#[from] ReadError),
}

#[derive(Debug, Error)]
pub enum InstallError {}

#[derive(Debug, Error)]
pub enum UninstallError {}

#[derive(Debug, Error)]
pub enum EnableError {}

#[derive(Debug, Error)]
pub enum DisableError {}

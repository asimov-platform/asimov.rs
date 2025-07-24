// This is free and unencumbered software released into the public domain.

use std::{io, path::PathBuf, string::String};

use reqwest::StatusCode;
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
    #[error("failed to read manifest at `{}`: {}", .0.display(), .1)]
    ReadManifestError(PathBuf, #[source] ReadManifestError),
}

#[derive(Debug, Error)]
pub enum ReadManifestError {
    #[error("failed to access module manifest file: {0}")]
    InstalledManifestIo(#[from] io::Error),
    #[error("failed to deserialize module manifest: {0}")]
    ManifestDeserialize(#[from] DeserializeError),
    #[error("unknown manifest format: {}", .0.as_deref().unwrap_or("no file extension"))]
    UnknownManifestFormat(Option<String>),
}

impl From<serde_json::Error> for ReadManifestError {
    fn from(value: serde_json::Error) -> Self {
        ReadManifestError::ManifestDeserialize(DeserializeError::Json(value))
    }
}

impl From<serde_yml::Error> for ReadManifestError {
    fn from(value: serde_yml::Error) -> Self {
        ReadManifestError::ManifestDeserialize(DeserializeError::Yaml(value))
    }
}

#[derive(Debug, Error)]
pub enum DeserializeError {
    #[error("JSON deserialization failed: {0}")]
    Json(#[from] serde_json::Error),
    #[error("YAML deserialization failed: {0}")]
    Yaml(#[from] serde_yml::Error),
}

#[derive(Debug, Error)]
pub enum FetchReleaseError {
    #[error("HTTP request failed: {0}")]
    Http(#[from] reqwest::Error),
    #[error("HTTP status code was not successful: {0}")]
    NotSuccess(StatusCode),
    #[error("unable to deserialize GitHub API response: {0}")]
    Deserialize(reqwest::Error),
}

#[derive(Debug, Error)]
pub enum ReadModuleVersionError {
    #[error(transparent)]
    ReadError(#[from] ReadManifestError),
    #[error("module is not installed")]
    NotInstalled,
}

#[derive(Debug, Error)]
pub enum InstallError {}

#[derive(Debug, Error)]
pub enum UninstallError {
    #[error("unable to read module manifest: {0}")]
    Manifest(#[from] ReadManifestError),
    #[error("unable to remove installed module file `{}`: {}", .0.display(), .1)]
    Io(PathBuf, #[source] io::Error),
    #[error("module is not installed")]
    NotInstalled,
}

#[derive(Debug, Error)]
pub enum EnableError {
    #[error(transparent)]
    ReadError(#[from] ReadManifestError),
    #[error("module is not installed")]
    NotInstalled,
    #[error("failed to enable module: {0}")]
    Io(#[from] io::Error),
}

#[derive(Debug, Error)]
pub enum DisableError {
    #[error("failed to disable module: {0}")]
    Io(#[from] io::Error),
}

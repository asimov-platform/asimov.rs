// This is free and unencumbered software released into the public domain.

use super::platform::PlatformInfo;
use std::{
    io,
    path::PathBuf,
    string::{String, ToString as _},
};
use thiserror::Error;

#[derive(Debug, Error)]
pub enum InstallError {
    #[error("failed to create directory for downloading: {0}")]
    CreateTempDir(io::Error),
    #[error(transparent)]
    Preinstall(#[from] PreinstallError),
    #[error(transparent)]
    Finish(#[from] FinishInstallError),
}

#[derive(Debug, Error)]
pub enum UpgradeError {
    #[error("unable to read current version of module: {0}")]
    CheckVersion(#[from] crate::registry::error::ModuleVersionError),
    #[error("failed to create directory for downloading: {0}")]
    CreateTempDir(io::Error),
    #[error("unable to check if module is enabled: {0}")]
    CheckEnabled(#[from] crate::registry::error::IsModuleEnabledError),
    #[error(transparent)]
    Preinstall(#[from] PreinstallError),
    #[error(transparent)]
    Uninstall(#[from] UninstallError),
    #[error(transparent)]
    Install(#[from] FinishInstallError),
    #[error("failed to re-enable module: {0}")]
    ReEnable(#[from] crate::registry::error::EnableError),
}

#[derive(Debug, Error)]
pub enum UninstallError {
    #[error("error while searching for manifest file: {0}")]
    FindManifest(#[from] crate::registry::error::FindManifestError),
    #[error("unable to read module manifest file: {0}")]
    Read(#[from] crate::registry::error::ReadManifestError),
    #[error(transparent)]
    Disable(#[from] crate::registry::error::DisableError),
    #[error("unable to remove installed module file `{0}`: {1}")]
    Delete(PathBuf, io::Error),
    #[error("module is not installed")]
    NotInstalled,
}

mod common {
    use super::*;

    #[derive(Debug, Error)]
    pub enum FetchError {
        #[error(transparent)]
        Http(#[from] HttpError),
        #[error("unable to deserialize GitHub API response: {0}")]
        Deserialize(#[from] DeserializeError),
    }

    impl From<reqwest::Error> for FetchError {
        fn from(value: reqwest::Error) -> Self {
            FetchError::Http(HttpError::Http(value))
        }
    }

    #[derive(Debug, Error)]
    pub enum DeserializeError {
        #[error("JSON deserialization failed: {0}")]
        Json(#[from] serde_json::Error),
        #[error("YAML deserialization failed: {0}")]
        Yaml(#[from] serde_yaml_ng::Error),
    }

    #[derive(Debug, Error)]
    pub enum FetchChecksumError {
        #[error(transparent)]
        Http(#[from] HttpError),
    }

    impl From<reqwest::Error> for FetchChecksumError {
        fn from(value: reqwest::Error) -> Self {
            FetchChecksumError::Http(HttpError::Http(value))
        }
    }

    #[derive(Debug, Error)]
    pub enum VerifyChecksumError {
        #[error("failed to read target file: {0}")]
        Io(#[from] io::Error),
        #[error("invalid checksum `{0}`, expected `{1}`")]
        InvalidChecksum(String, String),
    }

    #[derive(Debug, Error)]
    pub enum DownloadError {
        #[error(transparent)]
        Http(#[from] HttpError),
        #[error("failed to write data on disk: {0}")]
        Io(#[from] io::Error),
    }

    impl From<reqwest::Error> for DownloadError {
        fn from(value: reqwest::Error) -> Self {
            DownloadError::Http(HttpError::Http(value))
        }
    }

    #[derive(Debug, Error)]
    pub enum HttpError {
        #[error("HTTP request failed: {0}")]
        Http(#[from] reqwest::Error),
        #[error("HTTP status code was not successful: {0}")]
        NotSuccess(reqwest::StatusCode),
    }

    #[derive(Debug, Error)]
    pub enum PreinstallError {
        #[error("failed to create directory for installed manifests: {0}")]
        CreateManifestDir(io::Error),
        #[error("failed to create directory for installed binaries: {0}")]
        CreateExecDir(io::Error),
        #[error("failed to create directory for extracting: {0}")]
        CreateExtractDir(io::Error),

        #[error("no binaries available for platform `{}-{}{}`", .0.os, .0.arch, if let Some(ref libc) = .0.libc { "-".to_string() + libc } else { "".to_string() })]
        NotAvailable(PlatformInfo),

        #[error(transparent)]
        Download(#[from] DownloadError),
        #[error("failed to fetch module manifest: {0}")]
        FetchManifest(FetchError),

        #[error(transparent)]
        FetchChecksum(#[from] FetchChecksumError),
        #[error(transparent)]
        VerifyChecksum(#[from] VerifyChecksumError),

        #[error("failed to extract archive: {0}")]
        Extract(io::Error),
    }

    #[derive(Debug, Error)]
    pub enum FinishInstallError {
        #[error("failed to install binaries: {0}")]
        BinaryInstall(io::Error),
        #[error("failed to serialize module manifest: {0}")]
        SerializeManifest(#[from] serde_json::Error),
        #[error("failed to save module manifest: {0}")]
        SaveManifest(io::Error),
    }
}
pub use common::*;

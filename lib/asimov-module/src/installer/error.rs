// This is free and unencumbered software released into the public domain.

use std::{
    io,
    path::PathBuf,
    string::{String, ToString as _},
};

use thiserror::Error;

use super::platform::PlatformInfo;

#[derive(Debug, Error)]
pub enum CreateFileTreeError {
    #[error("failed to create directory for installed modules `{0}`: {1}")]
    InstallDir(PathBuf, #[source] io::Error),
    #[error("failed to create directory for enabled modules `{0}`: {1}")]
    EnableDir(PathBuf, #[source] io::Error),
    #[error("failed to create directory for module binaries `{0}`: {1}")]
    ExecDir(PathBuf, #[source] io::Error),
}

#[derive(Debug, Error)]
pub enum ReadError {
    #[error("failed to read directory for installed modules `{0}`: {1}")]
    InstallDirIo(PathBuf, #[source] io::Error),
    #[error("failed to read directory for enabled modules `{0}`: {1}")]
    EnableDirIo(PathBuf, #[source] io::Error),
    #[error("failed to read symlink for enabled module at `{0}`: {1}")]
    EnabledLinkIo(PathBuf, #[source] io::Error),
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
pub enum FetchChecksumError {
    #[error(transparent)]
    Http(#[from] HttpError),
}

#[derive(Debug, Error)]
pub enum VerifyChecksumError {
    #[error("failed to read target file: {0}")]
    Io(#[from] io::Error),
    #[error("invalid checksum `{0}`, expected `{1}`")]
    InvalidChecksum(String, String),
}

impl From<reqwest::Error> for FetchChecksumError {
    fn from(value: reqwest::Error) -> Self {
        FetchChecksumError::Http(HttpError::Http(value))
    }
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
pub enum ReadModuleVersionError {
    #[error(transparent)]
    ReadError(#[from] ReadManifestError),
    #[error("module is not installed")]
    NotInstalled,
}

#[derive(Debug, Error)]
pub enum InstallError {
    #[error("failed to create directory for installed manifests: {0}")]
    CreateManifestDir(io::Error),
    #[error("failed to create directory for installed binaries: {0}")]
    CreateExecDir(io::Error),
    #[error("failed to create directory for downloading: {0}")]
    CreateTempDir(io::Error),
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
    #[error("failed to install binaries: {0}")]
    BinaryInstall(io::Error),
    #[error("failed to serialize module manifest: {0}")]
    SerializeManifest(#[from] serde_json::Error),
    #[error("failed to save module manifest: {0}")]
    SaveManifest(io::Error),
}

#[derive(Debug, Error)]
pub enum UpgradeError {
    #[error(transparent)]
    Read(#[from] ReadManifestError),
    #[error("module is not installed")]
    NotInstalled,
    #[error(transparent)]
    Predownload(InstallError),
    #[error(transparent)]
    Uninstall(#[from] UninstallError),
    #[error(transparent)]
    Install(InstallError),
}

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

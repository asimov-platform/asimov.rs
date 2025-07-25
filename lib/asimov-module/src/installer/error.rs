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
pub enum InstalledModulesError {
    #[error("failed to read directory for installed modules `{0}`: {1}")]
    DirIo(PathBuf, #[source] io::Error),
    #[error("failed to read manifest at `{0}`: {1}")]
    ReadManifestError(PathBuf, #[source] ReadManifestError),
}

#[derive(Debug, Error)]
pub enum EnabledModulesError {
    #[error("failed to create directory for enabled modules `{0}`: {1}")]
    DirIo(PathBuf, #[source] io::Error),
    #[error("failed to read symlink for enabled module at `{0}`: {1}")]
    LinkIo(PathBuf, #[source] io::Error),
    #[error("failed to read manifest at `{0}`: {1}")]
    ReadManifestError(PathBuf, #[source] ReadManifestError),
}

#[derive(Debug, Error)]
#[error("error while searching for manifest file: {0}")]
pub struct IsModuleInstalledError(#[from] FindManifestError);

#[derive(Debug, Error)]
#[error("unable to read symlink for enabled module: {0}")]
pub struct IsModuleEnabledError(#[from] io::Error);

#[derive(Debug, Error)]
#[error(transparent)]
pub struct ModuleVersionError(#[from] ManifestError);

#[derive(Debug, Error)]
pub enum ManifestError {
    #[error("module is not installed")]
    NotInstalled,
    #[error("error while searching for manifest file: {0}")]
    FindManifest(#[from] FindManifestError),
    #[error("unable to read module manifest: {0}")]
    Read(#[from] ReadManifestError),
}

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
    CheckVersion(#[from] ModuleVersionError),
    #[error("failed to create directory for downloading: {0}")]
    CreateTempDir(io::Error),
    #[error("unable to check if module is enabled: {0}")]
    CheckEnabled(#[from] IsModuleEnabledError),
    #[error(transparent)]
    Preinstall(#[from] PreinstallError),
    #[error(transparent)]
    Uninstall(#[from] UninstallError),
    #[error(transparent)]
    Install(#[from] FinishInstallError),
    #[error("failed to re-enable module: {0}")]
    ReEnable(#[from] EnableError),
}

#[derive(Debug, Error)]
pub enum UninstallError {
    #[error("error while searching for manifest file: {0}")]
    FindManifest(#[from] FindManifestError),
    #[error("unable to read module manifest file: {0}")]
    Read(#[from] ReadManifestError),
    #[error(transparent)]
    Disable(#[from] DisableError),
    #[error("unable to remove installed module file `{0}`: {1}")]
    Delete(PathBuf, io::Error),
    #[error("module is not installed")]
    NotInstalled,
}

#[derive(Debug, Error)]
pub enum EnableError {
    #[error("error while searching for manifest file: {0}")]
    FindManifest(#[from] FindManifestError),
    #[error("module is not installed")]
    NotInstalled,
    #[error("failed to enable module: {0}")]
    Io(#[from] io::Error),
}

#[derive(Debug, Error)]
#[error("failed to disable module: {0}")]
pub struct DisableError(#[from] pub io::Error);

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

    #[derive(Debug, Error)]
    #[error("unable to check for manifest file at `{0}`: {1}")]
    pub struct FindManifestError(pub PathBuf, #[source] pub io::Error);
}
pub use common::*;

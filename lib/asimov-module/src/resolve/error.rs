// This is free and unencumbered software released into the public domain.

use alloc::string::String;

#[cfg(feature = "std")]
#[derive(Debug, thiserror::Error)]
pub enum FromDirError {
    #[error("failed to read manifest directory `{path}`: {source}")]
    ManifestDirIo {
        path: std::path::PathBuf,
        #[source]
        source: std::io::Error,
    },
    #[error("failed to read manifest file `{path}`: {source}")]
    ManifestIo {
        path: std::path::PathBuf,
        #[source]
        source: std::io::Error,
    },
    #[error("failed to parse manifest file `{path}`: {source}")]
    Parse {
        path: std::path::PathBuf,
        #[source]
        source: serde_yml::Error,
    },
    #[error("failed to add manifest file `{path}` to resolver: {source}")]
    Insert {
        path: std::path::PathBuf,
        #[source]
        source: UrlParseError,
    },
}

#[derive(Clone, Debug, thiserror::Error)]
pub enum UrlParseError {
    #[error("URL can't be empty")]
    EmptyUrl,
    #[error("invalid URL `{url}`: {source}")]
    InvalidUrl {
        url: String,
        #[source]
        source: url::ParseError,
    },
}

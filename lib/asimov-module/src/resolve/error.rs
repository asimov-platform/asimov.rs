// This is free and unencumbered software released into the public domain.

use alloc::string::String;

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

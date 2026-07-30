// This is free and unencumbered software released into the public domain.

use thiserror::Error;

#[derive(Clone, Debug, Eq, Error, PartialEq)]
pub enum KeyError {
    #[error("empty input")]
    EmptyInput,

    #[error("invalid length")]
    InvalidLength,

    #[error("invalid prefix")]
    InvalidPrefix,

    #[error("invalid encoding: {0}")]
    InvalidEncoding(bs58::decode::Error),
}

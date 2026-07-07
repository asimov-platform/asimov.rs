// This is free and unencumbered software released into the public domain.

use crate::{HandleError, KeyError};
use thiserror::Error;

#[derive(Clone, Debug, Eq, Error, PartialEq)]
pub enum IdError {
    #[error("empty input")]
    EmptyInput,

    #[error("invalid handle: {0}")]
    InvalidHandle(#[from] HandleError),

    #[error("invalid public key: {0}")]
    InvalidPublicKey(#[from] KeyError),
}

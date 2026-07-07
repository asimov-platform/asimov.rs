// This is free and unencumbered software released into the public domain.

use thiserror::Error;

#[derive(Clone, Debug, Eq, Error, Hash, Ord, PartialEq, PartialOrd)]
pub enum HandleError {
    #[error("empty input")]
    EmptyInput,

    #[error("invalid length: {0}")]
    InvalidLength(usize),

    #[error("invalid first character: {0}")]
    InvalidFirstChar(char),

    #[error("invalid character: {0}")]
    InvalidChar(char),
}

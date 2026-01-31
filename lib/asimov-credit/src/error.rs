// This is free and unencumbered software released into the public domain.

use thiserror::Error;

#[derive(Clone, Debug, Error, PartialEq)]
pub enum CreditsError {
    #[error("parse error: {0}")]
    Parse(rust_decimal::Error),
}

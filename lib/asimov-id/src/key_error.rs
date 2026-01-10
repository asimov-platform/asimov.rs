// This is free and unencumbered software released into the public domain.

use derive_more::Display;

#[derive(Clone, Debug, Display, Eq, PartialEq)]
pub enum KeyError {
    EmptyInput,
    InvalidLength,
    InvalidPrefix,
    InvalidEncoding(bs58::decode::Error),
}

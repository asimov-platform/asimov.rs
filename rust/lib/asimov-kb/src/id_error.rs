// This is free and unencumbered software released into the public domain.

use derive_more::Display;

#[derive(Clone, Debug, Display, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub enum IdError {
    InvalidLength,
    InvalidEncoding,
    UnknownClass,
}

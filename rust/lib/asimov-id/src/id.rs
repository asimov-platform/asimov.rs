// This is free and unencumbered software released into the public domain.

use crate::{Handle, IdError, KeyError, PublicKey};
use core::str::FromStr;

pub const ID_PREFIX: &str = "Ⓐ";

#[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub enum Id {
    Handle(Handle),
    PublicKey(PublicKey),
}

impl FromStr for Id {
    type Err = IdError;

    fn from_str(input: &str) -> Result<Self, Self::Err> {
        Ok(PublicKey::from_str(input)
            .map(|key| Id::PublicKey(key))
            .or_else(|_| Handle::from_str(input).map(|handle| Id::Handle(handle)))?)
    }
}

impl core::fmt::Display for Id {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        match self {
            Id::Handle(handle) => handle.fmt(f),
            Id::PublicKey(key) => key.fmt(f),
        }
    }
}

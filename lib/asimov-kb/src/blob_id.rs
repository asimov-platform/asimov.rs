// This is free and unencumbered software released into the public domain.

use crate::{Id, IdClass, IdError};
use alloc::vec::Vec;
use core::{ops::RangeInclusive, str::FromStr};
use derive_more::Display;

#[derive(Clone, Debug, Display, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct BlobId(pub(crate) Id<32>);

impl BlobId {
    pub const ID_LEN_MIN: usize = 1 + 16; // TODO
    pub const ID_LEN_MAX: usize = 1 + 22; // TODO
    pub const ID_LEN: RangeInclusive<usize> = Self::ID_LEN_MIN..=Self::ID_LEN_MAX;

    pub fn as_id(&self) -> &Id<32> {
        &self.0
    }

    pub fn into_id(self) -> Id<32> {
        self.0
    }
}

impl FromStr for BlobId {
    type Err = IdError;

    fn from_str(input: &str) -> Result<Self, Self::Err> {
        let id = Id::from_str(input)?;
        if id.class() != IdClass::Blob {
            return Err(IdError::UnknownClass);
        }
        Ok(Self(id))
    }
}

impl From<[u8; 32]> for BlobId {
    fn from(bytes: [u8; 32]) -> Self {
        Self(Id::from((IdClass::Blob, bytes)))
    }
}

impl From<&[u8; 32]> for BlobId {
    fn from(bytes: &[u8; 32]) -> Self {
        Self(Id::from((IdClass::Blob, bytes.clone())))
    }
}

impl From<&Vec<u8>> for BlobId {
    fn from(bytes: &Vec<u8>) -> Self {
        Self(Id::from((IdClass::Blob, bytes)))
    }
}

#[cfg(feature = "iroh")]
impl From<&iroh_blobs::Hash> for BlobId {
    fn from(bytes: &iroh_blobs::Hash) -> Self {
        Self(Id::from((IdClass::Blob, bytes.as_bytes().clone())))
    }
}

#[cfg(feature = "p2panda")]
impl From<&p2panda_core::Hash> for BlobId {
    fn from(bytes: &p2panda_core::Hash) -> Self {
        Self(Id::from((IdClass::Blob, bytes.as_bytes().clone())))
    }
}

#[cfg(feature = "iroh")]
impl Into<iroh_blobs::Hash> for BlobId {
    fn into(self) -> iroh_blobs::Hash {
        iroh_blobs::Hash::from(self.into_id().into_bytes())
    }
}

#[cfg(feature = "p2panda")]
impl Into<p2panda_core::Hash> for BlobId {
    fn into(self) -> p2panda_core::Hash {
        p2panda_core::Hash::from(self.into_id().into_bytes())
    }
}

#[cfg(feature = "eloquent")]
impl eloquent::ToSql for BlobId {
    fn to_sql(&self) -> Result<String, eloquent::error::EloquentError> {
        self.as_id().to_sql()
    }
}

#[cfg(feature = "libsql")]
impl libsql::params::IntoValue for BlobId {
    fn into_value(self) -> libsql::Result<libsql::Value> {
        self.into_id().into_value()
    }
}

#[cfg(feature = "rocket")]
impl<'r> rocket::request::FromParam<'r> for BlobId {
    type Error = IdError;

    fn from_param(input: &'r str) -> Result<Self, Self::Error> {
        Self::from_str(input)
    }
}

#[cfg(feature = "turso")]
impl turso::IntoValue for BlobId {
    fn into_value(self) -> turso::Result<turso::Value> {
        self.into_id().into_value()
    }
}

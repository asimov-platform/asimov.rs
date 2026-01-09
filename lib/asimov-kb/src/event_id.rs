// This is free and unencumbered software released into the public domain.

use crate::{Id, IdClass, IdError};
use core::{ops::RangeInclusive, str::FromStr};
use derive_more::Display;

#[derive(Clone, Debug, Display, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct EventId(pub(crate) Id<16>);

impl EventId {
    pub const ID_LEN_MIN: usize = 1 + 16;
    pub const ID_LEN_MAX: usize = 1 + 22;
    pub const ID_LEN: RangeInclusive<usize> = Self::ID_LEN_MIN..=Self::ID_LEN_MAX;

    #[cfg(feature = "uuid")]
    pub fn new() -> Self {
        Self(Id::new_uuid(IdClass::Event))
    }

    pub fn as_id(&self) -> &Id {
        &self.0
    }

    pub fn into_id(self) -> Id {
        self.0
    }
}

impl FromStr for EventId {
    type Err = IdError;

    fn from_str(input: &str) -> Result<Self, Self::Err> {
        let id = Id::from_str(input)?;
        if id.class() != IdClass::Event {
            return Err(IdError::UnknownClass);
        }
        Ok(Self(id))
    }
}

impl From<[u8; 16]> for EventId {
    fn from(bytes: [u8; 16]) -> Self {
        Self(Id::from((IdClass::Event, bytes)))
    }
}

impl From<&Vec<u8>> for EventId {
    fn from(bytes: &Vec<u8>) -> Self {
        Self(Id::from((IdClass::Event, bytes)))
    }
}

#[cfg(feature = "eloquent")]
impl eloquent::ToSql for EventId {
    fn to_sql(&self) -> Result<String, eloquent::error::EloquentError> {
        self.as_id().to_sql()
    }
}

#[cfg(feature = "libsql")]
impl libsql::params::IntoValue for EventId {
    fn into_value(self) -> libsql::Result<libsql::Value> {
        self.into_id().into_value()
    }
}

#[cfg(feature = "rocket")]
impl<'r> rocket::request::FromParam<'r> for EventId {
    type Error = IdError;

    fn from_param(input: &'r str) -> Result<Self, Self::Error> {
        Self::from_str(input)
    }
}

#[cfg(feature = "turso")]
impl turso::IntoValue for EventId {
    fn into_value(self) -> turso::Result<turso::Value> {
        self.into_id().into_value()
    }
}

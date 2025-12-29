// This is free and unencumbered software released into the public domain.

use crate::{Id, IdClass, IdError};
use core::str::FromStr;
use derive_more::Display;

#[derive(Clone, Debug, Display, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct PersonId(pub(crate) Id);

impl PersonId {
    pub fn new() -> Self {
        Self(Id::new(IdClass::Person))
    }

    pub fn as_id(&self) -> &Id {
        &self.0
    }
}

impl From<[u8; 16]> for PersonId {
    fn from(bytes: [u8; 16]) -> Self {
        Self(Id::from((IdClass::Person, bytes)))
    }
}

impl From<&Vec<u8>> for PersonId {
    fn from(bytes: &Vec<u8>) -> Self {
        Self(Id::from((IdClass::Person, bytes)))
    }
}

impl FromStr for PersonId {
    type Err = IdError;

    fn from_str(input: &str) -> Result<Self, Self::Err> {
        let id = Id::from_str(input)?;
        if id.class() != IdClass::Person {
            return Err(IdError::UnknownClass);
        }
        Ok(Self(id))
    }
}

#[cfg(feature = "eloquent")]
impl eloquent::ToSql for PersonId {
    fn to_sql(&self) -> Result<String, eloquent::error::EloquentError> {
        self.as_id().to_sql()
    }
}

#[cfg(feature = "rocket")]
impl<'r> rocket::request::FromParam<'r> for PersonId {
    type Error = IdError;

    fn from_param(input: &'r str) -> Result<Self, Self::Error> {
        Self::from_str(input)
    }
}

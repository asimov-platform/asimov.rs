// This is free and unencumbered software released into the public domain.

use crate::{Id, IdClass, IdError};
use core::str::FromStr;
use derive_more::Display;

#[derive(Clone, Debug, Display, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct EventId(pub(crate) Id);

impl EventId {
    pub fn as_id(&self) -> &Id {
        &self.0
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

#[cfg(feature = "rocket")]
impl<'r> rocket::request::FromParam<'r> for EventId {
    type Error = IdError;

    fn from_param(input: &'r str) -> Result<Self, Self::Error> {
        Self::from_str(input)
    }
}

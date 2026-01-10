// This is free and unencumbered software released into the public domain.

//! ASIMOV account IDs.

use crate::IdError;
use core::{borrow::Borrow, ops::RangeInclusive, str::FromStr};
use derive_more::Display;

pub const ID_LEN_MIN: usize = 1;
pub const ID_LEN_MAX: usize = 63;
pub const ID_LEN: RangeInclusive<usize> = ID_LEN_MIN..=ID_LEN_MAX;

#[derive(Clone, Debug, Display, Eq, Hash, Ord, PartialEq, PartialOrd)]
#[display("{}", self.0)]
#[cfg_attr(feature = "serde", derive(serde::Deserialize, serde::Serialize))]
#[cfg_attr(feature = "serde", serde(try_from = "String", into = "String"))]
pub struct Id(pub(crate) String);

impl Id {
    pub fn validate(input: &str) -> Result<(), IdError> {
        if input.is_empty() {
            return Err(IdError::EmptyInput);
        }

        if input.starts_with('-') {
            return Err(IdError::InvalidFirstChar('-'));
        }

        input
            .chars()
            .find(|c| !matches!(c, 'a'..='z' | 'A'..='Z' | '0'..='9' | '_' | '-'))
            .map_or(Ok(()), |c| Err(IdError::InvalidChar(c)))?;

        if input.len() < ID_LEN_MIN || input.len() > ID_LEN_MAX {
            return Err(IdError::InvalidLength(input.len()));
        }

        Ok(())
    }

    pub fn as_bytes(&self) -> &[u8] {
        &self.0.as_bytes()
    }

    pub fn as_str(&self) -> &str {
        &self.0
    }

    pub fn as_string(&self) -> &String {
        &self.0
    }

    pub fn into_string(self) -> String {
        self.0
    }

    pub fn glyph(&self) -> &str {
        "Ⓐ"
    }

    pub fn to_string_with_glyph(&self) -> String {
        format!("Ⓐ{}", self)
    }
}

impl FromStr for Id {
    type Err = IdError;

    fn from_str(input: &str) -> Result<Self, Self::Err> {
        Self::validate(input)?;
        Ok(Self(input.into()))
    }
}

impl TryFrom<String> for Id {
    type Error = IdError;

    fn try_from(input: String) -> Result<Self, Self::Error> {
        Self::from_str(&input)
    }
}

impl AsRef<[u8]> for Id {
    fn as_ref(&self) -> &[u8] {
        self.as_bytes()
    }
}

impl Borrow<str> for Id {
    fn borrow(&self) -> &str {
        &self.0
    }
}

impl Into<String> for Id {
    fn into(self) -> String {
        self.into_string()
    }
}

#[cfg(feature = "eloquent")]
impl eloquent::ToSql for Id {
    fn to_sql(&self) -> Result<String, eloquent::error::EloquentError> {
        Ok(self.to_string())
    }
}

#[cfg(feature = "libsql")]
impl libsql::params::IntoValue for Id {
    fn into_value(self) -> libsql::Result<libsql::Value> {
        Ok(libsql::Value::Text(self.into_string()))
    }
}

#[cfg(feature = "rocket")]
impl<'r> rocket::request::FromParam<'r> for Id {
    type Error = IdError;

    fn from_param(input: &'r str) -> Result<Self, Self::Error> {
        Self::from_str(input)
    }
}

#[cfg(feature = "turso")]
impl turso::IntoValue for Id {
    fn into_value(self) -> turso::Result<turso::Value> {
        Ok(turso::Value::Text(self.into_string()))
    }
}

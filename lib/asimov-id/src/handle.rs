// This is free and unencumbered software released into the public domain.

//! ASIMOV handle.

use crate::HandleError;
use alloc::{format, string::String};
use core::{borrow::Borrow, ops::RangeInclusive, str::FromStr};
use derive_more::Display;

pub const HANDLE_LEN_MIN: usize = 1;
pub const HANDLE_LEN_MAX: usize = 63;
pub const HANDLE_LEN: RangeInclusive<usize> = HANDLE_LEN_MIN..=HANDLE_LEN_MAX;

#[derive(Clone, Debug, Display, Eq, Hash, Ord, PartialEq, PartialOrd)]
#[display("{}", self.0)]
#[cfg_attr(feature = "serde", derive(serde::Deserialize, serde::Serialize))]
#[cfg_attr(feature = "serde", serde(try_from = "String", into = "String"))]
pub struct Handle(pub(crate) String);

impl Handle {
    pub fn validate(input: &str) -> Result<(), HandleError> {
        if input.is_empty() {
            return Err(HandleError::EmptyInput);
        }

        if input.starts_with('-') {
            return Err(HandleError::InvalidFirstChar('-'));
        }

        input
            .chars()
            .find(|c| !matches!(c, 'a'..='z' | 'A'..='Z' | '0'..='9' | '_' | '-'))
            .map_or(Ok(()), |c| Err(HandleError::InvalidChar(c)))?;

        if input.len() < HANDLE_LEN_MIN || input.len() > HANDLE_LEN_MAX {
            return Err(HandleError::InvalidLength(input.len()));
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

impl FromStr for Handle {
    type Err = HandleError;

    fn from_str(input: &str) -> Result<Self, Self::Err> {
        Self::validate(input)?;
        Ok(Self(input.into()))
    }
}

impl TryFrom<String> for Handle {
    type Error = HandleError;

    fn try_from(input: String) -> Result<Self, Self::Error> {
        Self::from_str(&input)
    }
}

impl AsRef<[u8]> for Handle {
    fn as_ref(&self) -> &[u8] {
        self.as_bytes()
    }
}

impl Borrow<str> for Handle {
    fn borrow(&self) -> &str {
        &self.0
    }
}

impl Into<String> for Handle {
    fn into(self) -> String {
        self.into_string()
    }
}

#[cfg(feature = "eloquent")]
impl eloquent::ToSql for Handle {
    fn to_sql(&self) -> Result<String, eloquent::error::EloquentError> {
        use alloc::string::ToString;
        Ok(self.to_string())
    }
}

#[cfg(feature = "libsql")]
impl libsql::params::IntoValue for Handle {
    fn into_value(self) -> libsql::Result<libsql::Value> {
        Ok(libsql::Value::Text(self.into_string()))
    }
}

#[cfg(feature = "rocket")]
impl<'r> rocket::request::FromParam<'r> for Handle {
    type Error = HandleError;

    fn from_param(input: &'r str) -> Result<Self, Self::Error> {
        Self::from_str(input)
    }
}

#[cfg(feature = "turso")]
impl turso::IntoValue for Handle {
    fn into_value(self) -> turso::Result<turso::Value> {
        Ok(turso::Value::Text(self.into_string()))
    }
}

// This is free and unencumbered software released into the public domain.

//! ASIMOV public keys.

use crate::KeyError;
use core::{ops::RangeInclusive, str::FromStr};
use derive_more::Display;

pub const KEY_LEN_MIN: usize = 1 + 32;
pub const KEY_LEN_MAX: usize = 1 + 44;
pub const KEY_LEN: RangeInclusive<usize> = KEY_LEN_MIN..=KEY_LEN_MAX;

#[derive(Clone, Copy, Debug, Default, Display, Eq, Hash, Ord, PartialEq, PartialOrd)]
#[display("Ⓐ{}", bs58::encode(self.0).into_string())]
#[cfg_attr(feature = "serde", derive(serde::Deserialize, serde::Serialize))]
#[cfg_attr(feature = "serde", serde(try_from = "String", into = "String"))]
pub struct Key(pub(crate) [u8; 32]);

impl Key {
    pub fn as_bytes(&self) -> &[u8] {
        self.0.as_slice()
    }

    pub fn into_bytes(self) -> [u8; 32] {
        self.0
    }
}

impl FromStr for Key {
    type Err = KeyError;

    fn from_str(input: &str) -> Result<Self, Self::Err> {
        if input.is_empty() {
            return Err(KeyError::EmptyInput);
        }
        if !KEY_LEN.contains(&input.len()) {
            return Err(KeyError::InvalidLength);
        }
        if input.chars().next() != Some('Ⓐ') {
            return Err(KeyError::InvalidPrefix);
        }
        let mut output = [0u8; 32];
        let count = bs58::decode(&input['Ⓐ'.len_utf8()..])
            .onto(&mut output)
            .map_err(|e| KeyError::InvalidEncoding(e))?;
        if count != output.len() {
            return Err(KeyError::InvalidLength);
        }
        Ok(Self(output))
    }
}

impl From<[u8; 32]> for Key {
    fn from(input: [u8; 32]) -> Self {
        Self(input)
    }
}

impl From<&[u8; 32]> for Key {
    fn from(input: &[u8; 32]) -> Self {
        Self(input.clone())
    }
}

impl From<&Vec<u8>> for Key {
    fn from(input: &Vec<u8>) -> Self {
        let mut bytes = [0u8; 32];
        let len = bytes.len().min(input.len());
        bytes[..len].copy_from_slice(&input[..len]);
        Self(bytes)
    }
}

#[cfg(feature = "ed25519-dalek")]
impl From<&ed25519_dalek::VerifyingKey> for Key {
    fn from(bytes: &ed25519_dalek::VerifyingKey) -> Self {
        Self(bytes.as_bytes().clone())
    }
}

#[cfg(feature = "iroh")]
impl From<&iroh::PublicKey> for Key {
    fn from(bytes: &iroh::PublicKey) -> Self {
        Self(bytes.as_bytes().clone())
    }
}

#[cfg(feature = "p2panda")]
impl From<&p2panda_core::PublicKey> for Key {
    fn from(bytes: &p2panda_core::PublicKey) -> Self {
        Self(bytes.as_bytes().clone())
    }
}

impl TryFrom<String> for Key {
    type Error = KeyError;

    fn try_from(input: String) -> Result<Self, Self::Error> {
        Self::from_str(&input)
    }
}

impl AsRef<[u8]> for Key {
    fn as_ref(&self) -> &[u8] {
        self.as_bytes()
    }
}

impl Into<String> for Key {
    fn into(self) -> String {
        self.to_string()
    }
}

#[cfg(feature = "iroh")]
impl TryInto<iroh::PublicKey> for Key {
    type Error = iroh::KeyParsingError;

    fn try_into(self) -> Result<iroh::PublicKey, Self::Error> {
        iroh::PublicKey::from_bytes(&self.into_bytes())
    }
}

#[cfg(feature = "p2panda")]
impl TryInto<p2panda_core::PublicKey> for Key {
    type Error = p2panda_core::IdentityError;

    fn try_into(self) -> Result<p2panda_core::PublicKey, Self::Error> {
        p2panda_core::PublicKey::from_bytes(&self.into_bytes())
    }
}

#[cfg(feature = "eloquent")]
impl eloquent::ToSql for Key {
    fn to_sql(&self) -> Result<String, eloquent::error::EloquentError> {
        let hex: String = self.0.iter().map(|b| format!("{b:02X}")).collect();
        Ok(format!("X'{hex}'"))
    }
}

#[cfg(feature = "libsql")]
impl libsql::params::IntoValue for Key {
    fn into_value(self) -> libsql::Result<libsql::Value> {
        Ok(libsql::Value::Blob(self.0.to_vec()))
    }
}

#[cfg(feature = "rocket")]
impl<'r> rocket::request::FromParam<'r> for Key {
    type Error = KeyError;

    fn from_param(input: &'r str) -> Result<Self, Self::Error> {
        Self::from_str(input)
    }
}

#[cfg(feature = "turso")]
impl turso::IntoValue for Key {
    fn into_value(self) -> turso::Result<turso::Value> {
        Ok(turso::Value::Blob(self.0.to_vec()))
    }
}

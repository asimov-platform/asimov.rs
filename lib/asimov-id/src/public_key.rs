// This is free and unencumbered software released into the public domain.

//! ASIMOV public keys.

use crate::{KEY_LEN, KeyError};
use alloc::{
    string::{String, ToString},
    vec::Vec,
};
use core::str::FromStr;
use derive_more::Display;

#[derive(Clone, Copy, Debug, Default, Display, Eq, Hash, Ord, PartialEq, PartialOrd)]
#[display("Ⓐ{}", bs58::encode(self.0).into_string())]
#[cfg_attr(feature = "serde", derive(serde::Deserialize, serde::Serialize))]
#[cfg_attr(feature = "serde", serde(try_from = "String", into = "String"))]
pub struct PublicKey(pub(crate) [u8; 32]);

impl PublicKey {
    pub fn as_bytes(&self) -> &[u8] {
        self.0.as_slice()
    }

    pub fn into_bytes(self) -> [u8; 32] {
        self.0
    }
}

impl FromStr for PublicKey {
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

impl AsRef<[u8]> for PublicKey {
    fn as_ref(&self) -> &[u8] {
        self.as_bytes()
    }
}

impl<T> From<&T> for PublicKey
where
    T: Clone + Into<Self>,
{
    fn from(t: &T) -> Self {
        t.clone().into()
    }
}

impl From<[u8; 32]> for PublicKey {
    fn from(input: [u8; 32]) -> Self {
        Self(input)
    }
}

impl From<&Vec<u8>> for PublicKey {
    fn from(input: &Vec<u8>) -> Self {
        let mut bytes = [0u8; 32];
        let len = bytes.len().min(input.len());
        bytes[..len].copy_from_slice(&input[..len]);
        Self(bytes)
    }
}

#[cfg(feature = "ed25519-dalek")]
impl From<&ed25519_dalek::VerifyingKey> for PublicKey {
    fn from(input: &ed25519_dalek::VerifyingKey) -> Self {
        Self(input.as_bytes().clone())
    }
}

#[cfg(feature = "iroh")]
impl From<iroh::PublicKey> for PublicKey {
    fn from(input: iroh::PublicKey) -> Self {
        Self(input.as_bytes().clone())
    }
}

#[cfg(feature = "iroh")]
impl TryFrom<PublicKey> for iroh::PublicKey {
    type Error = iroh::KeyParsingError;

    fn try_from(input: PublicKey) -> Result<iroh::PublicKey, Self::Error> {
        iroh::PublicKey::from_bytes(&input.into_bytes())
    }
}

// #[cfg(feature = "p2panda")]
// impl From<&p2panda_core::PublicKey> for PublicKey {
//     fn from(bytes: &p2panda_core::PublicKey) -> Self {
//         Self(bytes.as_bytes().clone())
//     }
// }

// #[cfg(feature = "p2panda")]
// impl TryInto<p2panda_core::PublicKey> for PublicKey {
//     type Error = p2panda_core::IdentityError;

//     fn try_into(self) -> Result<p2panda_core::PublicKey, Self::Error> {
//         p2panda_core::PublicKey::from_bytes(&self.into_bytes())
//     }
// }

impl TryFrom<String> for PublicKey {
    type Error = KeyError;

    fn try_from(input: String) -> Result<Self, Self::Error> {
        Self::from_str(&input)
    }
}

impl From<PublicKey> for String {
    fn from(input: PublicKey) -> String {
        input.to_string()
    }
}

#[cfg(feature = "eloquent")]
impl eloquent::ToSql for PublicKey {
    fn to_sql(&self) -> Result<String, eloquent::error::EloquentError> {
        use alloc::format;
        let hex: String = self.0.iter().map(|b| format!("{b:02X}")).collect();
        Ok(format!("X'{hex}'"))
    }
}

#[cfg(feature = "libsql")]
impl libsql::params::IntoValue for PublicKey {
    fn into_value(self) -> libsql::Result<libsql::Value> {
        Ok(libsql::Value::Blob(self.0.to_vec()))
    }
}

#[cfg(feature = "rocket")]
impl<'r> rocket::request::FromParam<'r> for PublicKey {
    type Error = KeyError;

    fn from_param(input: &'r str) -> Result<Self, Self::Error> {
        Self::from_str(input)
    }
}

#[cfg(feature = "turso")]
impl turso::IntoValue for PublicKey {
    fn into_value(self) -> turso::Result<turso::Value> {
        Ok(turso::Value::Blob(self.0.to_vec()))
    }
}

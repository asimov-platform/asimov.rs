// This is free and unencumbered software released into the public domain.

use crate::{IdClass, IdError};
use core::{ops::RangeInclusive, str::FromStr};
use derive_more::Display;

pub const ID_LENGTH_MIN: usize = 1 + 16;
pub const ID_LENGTH_MAX: usize = 1 + 22;
pub const ID_LENGTH: RangeInclusive<usize> = ID_LENGTH_MIN..=ID_LENGTH_MAX;

#[derive(Clone, Debug, Display, Eq, Hash, Ord, PartialEq, PartialOrd)]
#[display("{class}{}", bs58::encode(bytes).into_string())]
pub struct Id {
    pub(crate) class: IdClass,
    pub(crate) bytes: [u8; 16],
}

impl Id {
    pub fn zero(class: IdClass) -> Self {
        Self {
            class,
            bytes: [0u8; 16],
        }
    }

    pub fn new(class: IdClass) -> Self {
        Self {
            class,
            bytes: uuid::Uuid::now_v7().into_bytes(),
        }
    }

    pub fn class(&self) -> IdClass {
        self.class
    }

    pub fn as_bytes(&self) -> &[u8] {
        self.bytes.as_slice()
    }

    pub fn as_uuid(&self) -> uuid::Uuid {
        uuid::Uuid::from_bytes(self.bytes)
    }

    pub fn into_bytes(self) -> [u8; 16] {
        self.bytes
    }

    pub fn into_uuid(self) -> uuid::Uuid {
        uuid::Uuid::from_bytes(self.bytes)
    }

    #[cfg(feature = "std")]
    pub fn yaml_path(&self) -> std::path::PathBuf {
        self.dir_path().join(self.class().yaml_path())
    }

    #[cfg(feature = "std")]
    pub fn dir_path(&self) -> std::path::PathBuf {
        self.class()
            .dir_path()
            .join(format!("{}/{}", self.shard(), self))
    }

    fn shard(&self) -> String {
        let id_str = bs58::encode(self.bytes).into_string();
        let id_len = id_str.chars().count();
        id_str
            .chars()
            .skip(id_len.saturating_sub(2))
            .collect::<String>()
            .to_lowercase()
    }
}

impl FromStr for Id {
    type Err = IdError;

    fn from_str(input: &str) -> Result<Self, Self::Err> {
        let class = IdClass::from_str(input)?;
        let mut id = Id::zero(class);
        use bs58::decode::Error::*;
        match bs58::decode(&input[1..]).onto(&mut id.bytes) {
            Ok(len) => {
                if len == id.bytes.len() {
                    Ok(id)
                } else {
                    Err(IdError::InvalidLength)
                }
            },
            Err(err) => Err(match err {
                BufferTooSmall => IdError::InvalidLength,
                InvalidCharacter { .. } | NonAsciiCharacter { .. } => IdError::InvalidEncoding,
                _ => unreachable!(),
            }),
        }
    }
}

#[cfg(feature = "rocket")]
impl<'r> rocket::request::FromParam<'r> for Id {
    type Error = IdError;

    fn from_param(input: &'r str) -> Result<Self, Self::Error> {
        Self::from_str(input)
    }
}

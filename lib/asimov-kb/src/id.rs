// This is free and unencumbered software released into the public domain.

use crate::{IdClass, IdError};
use core::{ops::RangeInclusive, str::FromStr};
use derive_more::Display;
use regex::Regex;

pub const ID_LENGTH_MIN: usize = 1 + 16;
pub const ID_LENGTH_MAX: usize = 1 + 22;
pub const ID_LENGTH: RangeInclusive<usize> = ID_LENGTH_MIN..=ID_LENGTH_MAX;
pub const ID_REGEX: &str = r"^[1-9A-HJ-NP-Za-km-z]+$";

#[derive(Clone, Debug, Display, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct Id(pub(crate) String);

impl Id {
    pub fn as_str(&self) -> &str {
        self.0.as_str()
    }

    pub fn class(&self) -> IdClass {
        match self.0.chars().next().unwrap() {
            'E' => IdClass::Event,
            'P' => IdClass::Person,
            _ => unreachable!(),
        }
    }

    #[cfg(feature = "std")]
    pub fn yaml_path(&self) -> std::path::PathBuf {
        self.dir_path().join(self.class().yaml_path())
    }

    #[cfg(feature = "std")]
    pub fn dir_path(&self) -> std::path::PathBuf {
        self.class()
            .dir_path()
            .join(format!("{}/{}", self.shard(), self.0))
    }

    fn shard(&self) -> String {
        let id_len = self.0.chars().count();
        self.0
            .chars()
            .skip(id_len.saturating_sub(2))
            .collect::<String>()
            .to_lowercase()
    }
}

impl FromStr for Id {
    type Err = IdError;

    fn from_str(input: &str) -> Result<Self, Self::Err> {
        #[cfg(feature = "std")]
        fn matches(input: &str) -> bool {
            use std::sync::LazyLock;
            static ID_MATCHER: LazyLock<Regex> = LazyLock::new(|| Regex::new(ID_REGEX).unwrap());
            ID_MATCHER.is_match(input)
        }
        #[cfg(not(feature = "std"))]
        fn matches(_input: &str) -> bool {
            true
        }

        if !ID_LENGTH.contains(&input.len()) {
            return Err(IdError::InvalidLength);
        }

        if !matches(&input[1..]) {
            return Err(IdError::InvalidEncoding);
        }

        let class_char = input.chars().nth(0).unwrap();
        if class_char != 'E' && class_char != 'P' {
            return Err(IdError::UnknownClass);
        }

        Ok(Id(input.into()))
    }
}

#[cfg(feature = "rocket")]
impl<'r> rocket::request::FromParam<'r> for Id {
    type Error = IdError;

    fn from_param(input: &'r str) -> Result<Self, Self::Error> {
        Self::from_str(input)
    }
}

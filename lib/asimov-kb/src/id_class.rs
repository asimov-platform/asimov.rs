// This is free and unencumbered software released into the public domain.

use crate::IdError;
use core::str::FromStr;
use derive_more::Display;

#[derive(Clone, Copy, Debug, Display, Eq, Hash, Ord, PartialEq, PartialOrd)]
#[cfg_attr(feature = "clap", derive(clap::ValueEnum))]
pub enum IdClass {
    #[display("E")]
    Event,
    #[display("P")]
    Person,
}

impl IdClass {
    #[cfg(feature = "std")]
    pub fn yaml_path(&self) -> std::path::PathBuf {
        match self {
            Self::Event => "event.yaml",
            Self::Person => "person.yaml",
        }
        .into()
    }

    #[cfg(feature = "std")]
    pub fn dir_path(&self) -> std::path::PathBuf {
        match self {
            Self::Event => "events",
            Self::Person => "people",
        }
        .into()
    }
}

impl FromStr for IdClass {
    type Err = IdError;

    fn from_str(input: &str) -> Result<Self, Self::Err> {
        Ok(match input.chars().next().unwrap_or_default() {
            'E' => Self::Event,
            'P' => Self::Person,
            _ => return Err(IdError::UnknownClass),
        })
    }
}

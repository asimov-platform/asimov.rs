// This is free and unencumbered software released into the public domain.

use derive_more::Display;

#[derive(Clone, Debug, Display, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub enum IdClass {
    Event,
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

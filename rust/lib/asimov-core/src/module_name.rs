// This is free and unencumbered software released into the public domain.

use alloc::{borrow::ToOwned, string::String};
use core::{borrow::Borrow, fmt, ops::Deref, str::FromStr};

/// The name of a module.
///
/// A name consists only of lowercase letters, digits and hyphens, starts with a letter, and is at
/// most 64 characters long. A trailing hyphen is refused as well, which the specification permits
/// but `asimov-module-kit` does not create.
///
/// See: <https://asimov-specs.github.io/module-manifest/#name-field>
#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "serde", serde(try_from = "String", into = "String"))]
pub struct ModuleName(String);

impl ModuleName {
    pub const MAX_LENGTH: usize = 64;

    pub fn as_str(&self) -> &str {
        &self.0
    }

    pub fn into_string(self) -> String {
        self.0
    }
}

impl TryFrom<String> for ModuleName {
    type Error = InvalidModuleName;

    fn try_from(name: String) -> Result<Self, Self::Error> {
        let valid = name.len() <= Self::MAX_LENGTH
            && name.starts_with(|c: char| c.is_ascii_lowercase())
            && !name.ends_with('-')
            && name
                .bytes()
                .all(|b| b.is_ascii_lowercase() || b.is_ascii_digit() || b == b'-');

        if valid {
            Ok(Self(name))
        } else {
            Err(InvalidModuleName(name))
        }
    }
}

impl TryFrom<&str> for ModuleName {
    type Error = InvalidModuleName;

    fn try_from(name: &str) -> Result<Self, Self::Error> {
        name.to_owned().try_into()
    }
}

impl FromStr for ModuleName {
    type Err = InvalidModuleName;

    fn from_str(name: &str) -> Result<Self, Self::Err> {
        name.try_into()
    }
}

impl From<ModuleName> for String {
    fn from(name: ModuleName) -> Self {
        name.0
    }
}

impl AsRef<str> for ModuleName {
    fn as_ref(&self) -> &str {
        &self.0
    }
}

impl Borrow<str> for ModuleName {
    fn borrow(&self) -> &str {
        &self.0
    }
}

impl Deref for ModuleName {
    type Target = str;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl fmt::Display for ModuleName {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(&self.0)
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct InvalidModuleName(pub String);

impl fmt::Display for InvalidModuleName {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "invalid module name `{}`", self.0)
    }
}

impl core::error::Error for InvalidModuleName {}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn accepts_module_names() {
        for name in [
            "ipfs",
            "search-google-fetcher",
            "x",
            "a1-b2",
            &"a".repeat(ModuleName::MAX_LENGTH),
        ] {
            assert!(name.parse::<ModuleName>().is_ok(), "{name}");
        }
    }

    #[test]
    fn rejects_invalid_names() {
        for name in [
            "",
            "0-leading-digit",
            "-sample",
            "Sample",
            "sample name",
            "sample_name",
            "sample-",
            &"a".repeat(ModuleName::MAX_LENGTH + 1),
        ] {
            assert!(name.parse::<ModuleName>().is_err(), "{name}");
        }
    }

    #[test]
    fn rejects_names_that_are_not_path_components() {
        for name in [
            "..",
            "../victim",
            "sample/../../victim",
            "a/b",
            "sample.json",
        ] {
            assert!(name.parse::<ModuleName>().is_err(), "{name}");
        }
    }
}

// This is free and unencumbered software released into the public domain.

use derive_more::Display;
use dogma::prelude::{FromStr, String};

#[derive(Clone, Debug, Default, Display, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct PromptMessage(pub String);

impl FromStr for PromptMessage {
    type Err = ();

    fn from_str(input: &str) -> Result<Self, Self::Err> {
        Ok(PromptMessage(input.into()))
    }
}

impl From<&str> for PromptMessage {
    fn from(input: &str) -> Self {
        PromptMessage(input.into())
    }
}

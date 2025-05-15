// This is free and unencumbered software released into the public domain.

use derive_more::{Display, From};
use dogma::prelude::{FromStr, String};

#[derive(Clone, Debug, Default, Display, Eq, From, Hash, Ord, PartialEq, PartialOrd)]
#[from(&str, String)]
pub struct PromptMessage(pub String);

impl FromStr for PromptMessage {
    type Err = ();

    fn from_str(input: &str) -> Result<Self, Self::Err> {
        Ok(input.into())
    }
}

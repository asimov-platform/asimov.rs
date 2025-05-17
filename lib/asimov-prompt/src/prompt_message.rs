// This is free and unencumbered software released into the public domain.

use super::prompt_role::PromptRole;
use derive_more::Display;
use dogma::prelude::{FromStr, String};

#[derive(Clone, Debug, Display, Eq, Hash, Ord, PartialEq, PartialOrd)]
#[display("{_0}: {_1}")]
pub struct PromptMessage(pub PromptRole, pub String);

impl FromStr for PromptMessage {
    type Err = ();

    fn from_str(input: &str) -> Result<Self, Self::Err> {
        Ok((PromptRole::User, input).into())
    }
}

impl From<&str> for PromptMessage {
    fn from(input: &str) -> Self {
        (PromptRole::User, input).into()
    }
}

impl From<String> for PromptMessage {
    fn from(input: String) -> Self {
        (PromptRole::User, input).into()
    }
}

impl From<(PromptRole, &str)> for PromptMessage {
    fn from((role, message): (PromptRole, &str)) -> Self {
        PromptMessage(role, message.into())
    }
}

impl From<(PromptRole, String)> for PromptMessage {
    fn from((role, message): (PromptRole, String)) -> Self {
        PromptMessage(role, message)
    }
}

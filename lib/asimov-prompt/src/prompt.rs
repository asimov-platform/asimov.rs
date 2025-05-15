// This is free and unencumbered software released into the public domain.

use super::{prompt_message::PromptMessage, prompt_role::PromptRole};
use dogma::{
    prelude::{FromStr, String, Vec, fmt},
    traits::Collection,
};
use typed_builder::TypedBuilder;

#[derive(Clone, Debug, Default, Eq, Hash, Ord, PartialEq, PartialOrd, TypedBuilder)]
pub struct Prompt {
    pub messages: Vec<(PromptRole, PromptMessage)>,
}

impl Collection for Prompt {
    type Item = (PromptRole, PromptMessage);

    fn len(&self) -> usize {
        self.messages.len()
    }
}

impl FromStr for Prompt {
    type Err = ();

    fn from_str(input: &str) -> Result<Self, Self::Err> {
        Ok(input.into())
    }
}

impl From<(PromptRole, &str)> for Prompt {
    fn from((role, message): (PromptRole, &str)) -> Self {
        let messages = Vec::from([(role, message.into())]);
        Prompt { messages }
    }
}

impl From<&str> for Prompt {
    fn from(input: &str) -> Self {
        (PromptRole::User, input).into()
    }
}

impl From<String> for Prompt {
    fn from(input: String) -> Self {
        (PromptRole::User, input.as_str()).into()
    }
}

impl fmt::Display for Prompt {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        for (role, message) in &self.messages {
            writeln!(f, "{}: {}", role, message)?;
        }
        Ok(())
    }
}

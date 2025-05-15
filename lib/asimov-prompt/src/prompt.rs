// This is free and unencumbered software released into the public domain.

use super::{prompt_message::PromptMessage, prompt_role::PromptRole};
use dogma::{
    prelude::{FromStr, Vec, fmt},
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
        let messages = Vec::from([(PromptRole::User, input.into())]);
        Ok(Prompt { messages })
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

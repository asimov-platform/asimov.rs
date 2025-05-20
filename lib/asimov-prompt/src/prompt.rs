// This is free and unencumbered software released into the public domain.

use super::{prompt_message::PromptMessage, prompt_role::PromptRole};
use dogma::{
    prelude::{FromStr, String, Vec, fmt},
    traits::Collection,
};
use typed_builder::TypedBuilder;

#[derive(Clone, Debug, Default, Eq, Hash, Ord, PartialEq, PartialOrd, TypedBuilder)]
pub struct Prompt {
    pub messages: Vec<PromptMessage>,
}

impl Collection for Prompt {
    type Item = PromptMessage;

    fn len(&self) -> usize {
        self.messages.len()
    }
}

impl From<Vec<PromptMessage>> for Prompt {
    fn from(messages: Vec<PromptMessage>) -> Self {
        Self { messages }
    }
}

impl FromStr for Prompt {
    type Err = ();

    fn from_str(input: &str) -> Result<Self, Self::Err> {
        Ok(input.into())
    }
}

impl From<&str> for Prompt {
    fn from(input: &str) -> Self {
        (PromptRole::User, input).into()
    }
}

impl From<String> for Prompt {
    fn from(input: String) -> Self {
        (PromptRole::User, input).into()
    }
}

impl From<(PromptRole, &str)> for Prompt {
    fn from((role, message): (PromptRole, &str)) -> Self {
        (role, String::from(message)).into()
    }
}

impl From<(PromptRole, String)> for Prompt {
    fn from((role, message): (PromptRole, String)) -> Self {
        Prompt {
            messages: Vec::from([(role, message).into()]),
        }
    }
}

impl fmt::Display for Prompt {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        for PromptMessage(role, message) in &self.messages {
            writeln!(f, "{}: {}", role, message)?;
        }
        Ok(())
    }
}

#[cfg(feature = "openai")]
impl TryFrom<openai::schemas::CreateCompletionRequest_Prompt> for Prompt {
    type Error = ();

    fn try_from(
        input: openai::schemas::CreateCompletionRequest_Prompt,
    ) -> Result<Self, Self::Error> {
        use openai::schemas::CreateCompletionRequest_Prompt::*;
        match input {
            Text(prompt) => Ok(prompt.into()),
            TextArray(prompts) => Ok(prompts.join("").into()),
            TokenArray(_) => Err(()),
            TokenArrayArray(_) => Err(()),
        }
    }
}

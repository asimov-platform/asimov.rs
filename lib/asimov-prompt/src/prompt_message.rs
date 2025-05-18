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

#[cfg(feature = "openai")]
impl TryFrom<openai::schemas::ChatCompletionRequestMessage> for PromptMessage {
    type Error = ();

    fn try_from(input: openai::schemas::ChatCompletionRequestMessage) -> Result<Self, Self::Error> {
        use PromptRole::*;
        match (input.role(), input.text_content()) {
            ("assistant", Some(message)) => Ok(PromptMessage(Assistant, message.into())),
            ("developer", Some(message)) => Ok(PromptMessage(Developer, message.into())),
            ("system", Some(message)) => Ok(PromptMessage(System, message.into())),
            ("user", Some(message)) => Ok(PromptMessage(User, message.into())),
            _ => Err(()),
        }
    }
}

#[cfg(feature = "openai")]
impl TryFrom<openai::schemas::ChatCompletionRequestAssistantMessage> for PromptMessage {
    type Error = ();

    fn try_from(
        input: openai::schemas::ChatCompletionRequestAssistantMessage,
    ) -> Result<Self, Self::Error> {
        input.content.ok_or(())?.try_into()
    }
}

#[cfg(feature = "openai")]
impl TryFrom<openai::schemas::ChatCompletionRequestAssistantMessage_Content> for PromptMessage {
    type Error = ();

    fn try_from(
        input: openai::schemas::ChatCompletionRequestAssistantMessage_Content,
    ) -> Result<Self, Self::Error> {
        let text_content = input.text_content().ok_or(())?;
        Ok(PromptMessage(PromptRole::Assistant, text_content.into()))
    }
}

#[cfg(feature = "openai")]
impl TryFrom<openai::schemas::ChatCompletionRequestDeveloperMessage> for PromptMessage {
    type Error = ();

    fn try_from(
        input: openai::schemas::ChatCompletionRequestDeveloperMessage,
    ) -> Result<Self, Self::Error> {
        input.content.try_into()
    }
}

#[cfg(feature = "openai")]
impl TryFrom<openai::schemas::ChatCompletionRequestDeveloperMessage_Content> for PromptMessage {
    type Error = ();

    fn try_from(
        input: openai::schemas::ChatCompletionRequestDeveloperMessage_Content,
    ) -> Result<Self, Self::Error> {
        let text_content = input.text_content().ok_or(())?;
        Ok(PromptMessage(PromptRole::Developer, text_content.into()))
    }
}

#[cfg(feature = "openai")]
impl TryFrom<openai::schemas::ChatCompletionRequestSystemMessage> for PromptMessage {
    type Error = ();

    fn try_from(
        input: openai::schemas::ChatCompletionRequestSystemMessage,
    ) -> Result<Self, Self::Error> {
        input.content.try_into()
    }
}

#[cfg(feature = "openai")]
impl TryFrom<openai::schemas::ChatCompletionRequestSystemMessage_Content> for PromptMessage {
    type Error = ();

    fn try_from(
        input: openai::schemas::ChatCompletionRequestSystemMessage_Content,
    ) -> Result<Self, Self::Error> {
        let text_content = input.text_content().ok_or(())?;
        Ok(PromptMessage(PromptRole::Developer, text_content.into()))
    }
}

#[cfg(feature = "openai")]
impl TryFrom<openai::schemas::ChatCompletionRequestUserMessage> for PromptMessage {
    type Error = ();

    fn try_from(
        input: openai::schemas::ChatCompletionRequestUserMessage,
    ) -> Result<Self, Self::Error> {
        input.content.try_into()
    }
}

#[cfg(feature = "openai")]
impl TryFrom<openai::schemas::ChatCompletionRequestUserMessage_Content> for PromptMessage {
    type Error = ();

    fn try_from(
        input: openai::schemas::ChatCompletionRequestUserMessage_Content,
    ) -> Result<Self, Self::Error> {
        let text_content = input.text_content().ok_or(())?;
        Ok(PromptMessage(PromptRole::User, text_content.into()))
    }
}

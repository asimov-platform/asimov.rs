// This is free and unencumbered software released into the public domain.

use crate::Execute;
use typed_builder::TypedBuilder;

pub use asimov_prompt::{Prompt, PromptMessage, PromptRole};

/// LLM inference provider. Consumes text input, produces text output.
pub trait Provider<T, E>: Execute<T, E> {}

/// Configuration options for [`Provider`].
///
/// # Examples
///
/// ```rust
/// use asimov_patterns::{ProviderOptions, Prompt, PromptMessage, PromptRole};
///
/// let options = ProviderOptions::builder()
///     .prompt(
///         Prompt::builder()
///             .messages(vec![PromptMessage(
///                 PromptRole::User,
///                 "Hello, world!".into(),
///             )])
///             .build(),
///     )
///     .build();
///
/// ```
#[derive(Clone, Debug, Default, Eq, Hash, Ord, PartialEq, PartialOrd, TypedBuilder)]
pub struct ProviderOptions {
    pub prompt: Prompt,
}

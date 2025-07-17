// This is free and unencumbered software released into the public domain.

use crate::Execute;
use alloc::{string::String, vec::Vec};
use typed_builder::TypedBuilder;

/// LLM inference provider. Consumes prompt input, produces response output.
///
/// See: https://asimov-specs.github.io/program-patterns/#prompter
pub trait Prompter<T, E>: Execute<T, E> {}

/// Configuration options for [`Prompter`].
///
/// # Examples
///
/// ```rust
/// use asimov_patterns::{PrompterOptions, Prompt, PromptMessage, PromptRole};
///
/// let options = PrompterOptions::builder()
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
pub struct PrompterOptions {
    /// The input format.
    pub input: Option<String>,

    /// The inference model.
    pub model: Option<String>,

    /// The output format.
    pub output: Option<String>,

    /// Extended nonstandard program options.
    pub other: Vec<String>,
}

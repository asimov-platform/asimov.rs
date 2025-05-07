// This is free and unencumbered software released into the public domain.

use typed_builder::TypedBuilder;

/// LLM inference provider. Consumes text input, produces text output.
pub trait Provider {}

/// Configuration options for [`Provider`].
///
/// # Examples
///
/// ```rust
/// use asimov_patterns::ProviderOptions;
///
/// let options = ProviderOptions::builder().build();
/// ```
#[derive(Clone, Debug, Default, Eq, Hash, Ord, PartialEq, PartialOrd, TypedBuilder)]
pub struct ProviderOptions {}

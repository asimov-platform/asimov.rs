// This is free and unencumbered software released into the public domain.

/// LLM inference provider. Consumes text input, produces text output.
pub trait Provider {}

/// Options for [`Provider`].
#[derive(Clone, Debug)]
pub struct ProviderOptions {}

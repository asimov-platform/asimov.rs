// This is free and unencumbered software released into the public domain.

use asimov_patterns::ProviderOptions;
use std::path::Path;

/// LLM inference provider. Consumes text input, produces text output.
#[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct Provider {}

impl asimov_patterns::Provider for Provider {}

impl Provider {
    pub fn new(_program: impl AsRef<Path>, _options: &ProviderOptions) -> Self {
        Self {}
    }
}

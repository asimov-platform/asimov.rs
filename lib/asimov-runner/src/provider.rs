// This is free and unencumbered software released into the public domain.

use asimov_patterns::ProviderOptions;
use std::ffi::OsStr;

/// LLM inference provider. Consumes text input, produces text output.
#[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct Provider {}

impl Provider {
    pub fn new(_program: impl AsRef<OsStr>, _options: ProviderOptions) -> Self {
        Self {}
    }
}

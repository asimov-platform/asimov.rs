// This is free and unencumbered software released into the public domain.

use std::ffi::OsStr;

pub use asimov_patterns::AdapterOptions;

/// RDF dataset adapter. Consumes SPARQL input, produces RDF output.
#[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct Adapter {}

impl Adapter {
    pub fn new(_program: impl AsRef<OsStr>, _options: AdapterOptions) -> Self {
        Self {}
    }
}

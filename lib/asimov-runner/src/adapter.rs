// This is free and unencumbered software released into the public domain.

use asimov_patterns::AdapterOptions;
use std::ffi::OsStr;

/// RDF dataset adapter. Consumes SPARQL input, produces RDF output.
#[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct Adapter {}

impl Adapter {
    pub fn new(_program: impl AsRef<OsStr>, _options: AdapterOptions) -> Self {
        Self {}
    }
}

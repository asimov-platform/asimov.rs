// This is free and unencumbered software released into the public domain.

use asimov_patterns::AdapterOptions;
use std::path::Path;

/// RDF dataset adapter. Consumes SPARQL input, produces RDF output.
#[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct Adapter {}

impl asimov_patterns::Adapter for Adapter {}

impl Adapter {
    pub fn new(_program: impl AsRef<Path>, _options: &AdapterOptions) -> Self {
        Self {}
    }
}

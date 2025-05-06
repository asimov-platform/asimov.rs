// This is free and unencumbered software released into the public domain.

use asimov_patterns::ReasonerOptions;
use std::path::Path;

/// RDF dataset reasoner. Consumes RDF input, produces entailed RDF output.
#[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct Reasoner {}

impl asimov_patterns::Reasoner for Reasoner {}

impl Reasoner {
    pub fn new(_program: impl AsRef<Path>, _options: &ReasonerOptions) -> Self {
        Self {}
    }
}

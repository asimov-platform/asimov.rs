// This is free and unencumbered software released into the public domain.

use asimov_patterns::ReasonerOptions;
use std::ffi::OsStr;

/// RDF dataset reasoner. Consumes RDF input, produces entailed RDF output.
#[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct Reasoner {}

impl Reasoner {
    pub fn new(_program: impl AsRef<OsStr>, _options: ReasonerOptions) -> Self {
        Self {}
    }
}

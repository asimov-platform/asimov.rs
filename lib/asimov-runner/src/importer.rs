// This is free and unencumbered software released into the public domain.

use asimov_patterns::ImporterOptions;
use std::path::Path;

/// RDF dataset importer. Consumes a URL input, produces RDF output.
#[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct Importer {}

impl asimov_patterns::Importer for Importer {}

impl Importer {
    pub fn new(_program: impl AsRef<Path>, _options: &ImporterOptions) -> Self {
        Self {}
    }
}

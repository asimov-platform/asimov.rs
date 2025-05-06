// This is free and unencumbered software released into the public domain.

use asimov_patterns::ReaderOptions;
use std::path::Path;

/// RDF dataset converter. Consumes some input, produces RDF output.
#[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct Reader {}

impl asimov_patterns::Reader for Reader {}

impl Reader {
    pub fn new(_program: impl AsRef<Path>, _options: &ReaderOptions) -> Self {
        Self {}
    }
}

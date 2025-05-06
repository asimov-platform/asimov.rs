// This is free and unencumbered software released into the public domain.

use asimov_patterns::WriterOptions;
use std::path::Path;

/// RDF dataset converter. Consumes RDF input, produces some output.
#[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct Writer {}

impl asimov_patterns::Writer for Writer {}

impl Writer {
    pub fn new(_program: impl AsRef<Path>, _options: &WriterOptions) -> Self {
        Self {}
    }
}

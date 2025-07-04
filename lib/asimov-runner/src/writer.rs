// This is free and unencumbered software released into the public domain.

use std::ffi::OsStr;

pub use asimov_patterns::WriterOptions;

/// RDF dataset converter. Consumes RDF input, produces some output.
#[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct Writer {}

impl Writer {
    pub fn new(_program: impl AsRef<OsStr>, _options: WriterOptions) -> Self {
        Self {}
    }
}

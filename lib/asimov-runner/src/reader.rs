// This is free and unencumbered software released into the public domain.

use std::ffi::OsStr;

pub use asimov_patterns::ReaderOptions;

/// RDF dataset converter. Consumes some input, produces RDF output.
#[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct Reader {}

impl Reader {
    pub fn new(_program: impl AsRef<OsStr>, _options: ReaderOptions) -> Self {
        Self {}
    }
}

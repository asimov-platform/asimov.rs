// This is free and unencumbered software released into the public domain.

use asimov_patterns::RegistrarOptions;
use std::ffi::OsStr;

/// Namespace registrar. Consumes a URL input, produces RDF output.
#[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct Registrar {}

impl Registrar {
    pub fn new(_program: impl AsRef<OsStr>, _options: RegistrarOptions) -> Self {
        Self {}
    }
}

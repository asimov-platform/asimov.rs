// This is free and unencumbered software released into the public domain.

use asimov_patterns::RegistrarOptions;
use std::path::Path;

/// Namespace registrar. Consumes a URL input, produces RDF output.
#[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct Registrar {}

impl asimov_patterns::Registrar for Registrar {}

impl Registrar {
    pub fn new(_program: impl AsRef<Path>, _options: &RegistrarOptions) -> Self {
        Self {}
    }
}

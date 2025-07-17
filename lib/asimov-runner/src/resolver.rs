// This is free and unencumbered software released into the public domain.

use std::ffi::OsStr;

pub use asimov_patterns::ResolverOptions;

/// See: https://asimov-specs.github.io/program-patterns/#resolver
#[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct Registrar {
    options: ResolverOptions,
}

impl Registrar {
    pub fn new(_program: impl AsRef<OsStr>, options: ResolverOptions) -> Self {
        Self { options }
    }
}

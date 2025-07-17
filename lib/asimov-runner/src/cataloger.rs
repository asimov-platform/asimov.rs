// This is free and unencumbered software released into the public domain.

use std::ffi::OsStr;

pub use asimov_patterns::CatalogerOptions;

/// See: https://asimov-specs.github.io/program-patterns/#cataloger
#[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct Cataloger {
    options: CatalogerOptions,
}

impl Cataloger {
    pub fn new(
        _program: impl AsRef<OsStr>,
        _input_url: impl AsRef<OsStr>,
        options: CatalogerOptions,
    ) -> Self {
        Self { options }
    }
}

// This is free and unencumbered software released into the public domain.

use std::ffi::OsStr;

pub use asimov_patterns::EmitterOptions;

/// See: https://asimov-specs.github.io/program-patterns/#emitter
#[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct Emitter {
    options: EmitterOptions,
}

impl Emitter {
    pub fn new(_program: impl AsRef<OsStr>, options: EmitterOptions) -> Self {
        Self { options }
    }
}

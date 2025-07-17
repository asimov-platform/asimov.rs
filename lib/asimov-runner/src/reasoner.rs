// This is free and unencumbered software released into the public domain.

use std::ffi::OsStr;

pub use asimov_patterns::ReasonerOptions;

/// See: https://asimov-specs.github.io/program-patterns/#reasoner
#[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct Reasoner {
    options: ReasonerOptions,
}

impl Reasoner {
    pub fn new(_program: impl AsRef<OsStr>, options: ReasonerOptions) -> Self {
        Self { options }
    }
}

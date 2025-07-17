// This is free and unencumbered software released into the public domain.

use std::ffi::OsStr;

pub use asimov_patterns::WriterOptions;

/// See: https://asimov-specs.github.io/program-patterns/#writer
#[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct Writer {
    options: WriterOptions,
}

impl Writer {
    pub fn new(_program: impl AsRef<OsStr>, options: WriterOptions) -> Self {
        Self { options }
    }
}

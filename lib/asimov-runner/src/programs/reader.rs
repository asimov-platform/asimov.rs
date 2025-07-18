// This is free and unencumbered software released into the public domain.

use std::ffi::OsStr;

pub use asimov_patterns::ReaderOptions;

/// See: https://asimov-specs.github.io/program-patterns/#reader
#[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct Reader {
    options: ReaderOptions,
}

impl Reader {
    pub fn new(_program: impl AsRef<OsStr>, options: ReaderOptions) -> Self {
        Self { options }
    }
}

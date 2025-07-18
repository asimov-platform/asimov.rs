// This is free and unencumbered software released into the public domain.

use std::ffi::OsStr;

pub use asimov_patterns::AdapterOptions;

pub type Query = String; // TODO

/// See: https://asimov-specs.github.io/program-patterns/#adapter
#[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct Adapter {
    query: Query,
    options: AdapterOptions,
}

impl Adapter {
    pub fn new(_program: impl AsRef<OsStr>, query: Query, options: AdapterOptions) -> Self {
        Self { query, options }
    }
}

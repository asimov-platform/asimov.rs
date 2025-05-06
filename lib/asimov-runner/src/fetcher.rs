// This is free and unencumbered software released into the public domain.

use asimov_patterns::FetcherOptions;
use std::path::Path;

/// Network protocol fetcher. Consumes a URL input, produces some output.
#[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct Fetcher {}

impl asimov_patterns::Fetcher for Fetcher {}

impl Fetcher {
    pub fn new(_program: impl AsRef<Path>, _options: &FetcherOptions) -> Self {
        Self {}
    }
}

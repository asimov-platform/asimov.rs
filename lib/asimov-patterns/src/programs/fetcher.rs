// This is free and unencumbered software released into the public domain.

use crate::Execute;
use alloc::{string::String, vec::Vec};
use bon::Builder;

/// URL resource materializer. Consumes a URL input, produces RDF output.
///
/// See: https://asimov-specs.github.io/program-patterns/#fetcher
pub trait Fetcher<T, E>: Execute<T, E> {}

/// Configuration options for [`Fetcher`].
///
/// # Examples
///
/// ```rust
/// use asimov_patterns::FetcherOptions;
///
/// let options = FetcherOptions::builder()
///     .output("jsonld")
///     .build();
/// ```
#[derive(Clone, Debug, Default, Eq, Hash, Ord, PartialEq, PartialOrd, Builder)]
#[builder(derive(Debug), on(String, into))]
pub struct FetcherOptions {
    /// Extended nonstandard fetcher options.
    #[builder(field)]
    pub other: Vec<String>,

    /// The output format.
    pub output: Option<String>,
}

impl<S: fetcher_options_builder::State> FetcherOptionsBuilder<S> {
    pub fn other(mut self, flag: impl Into<String>) -> Self {
        self.other.push(flag.into());
        self
    }

    pub fn maybe_other(mut self, flag: Option<impl Into<String>>) -> Self {
        if let Some(flag) = flag {
            self.other.push(flag.into());
        }
        self
    }
}

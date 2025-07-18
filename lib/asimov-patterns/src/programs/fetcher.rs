// This is free and unencumbered software released into the public domain.

use crate::Execute;
use alloc::{string::String, vec::Vec};
use typed_builder::TypedBuilder;

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
///     .input_url("https://crates.io/robots.txt".into())
///     .build();
/// ```
#[derive(Clone, Debug, Default, Eq, Hash, Ord, PartialEq, PartialOrd, TypedBuilder)]
pub struct FetcherOptions {
    /// The output format.
    pub output: Option<String>,

    /// Extended nonstandard fetcher options.
    pub other: Vec<String>,
}

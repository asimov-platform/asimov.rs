// This is free and unencumbered software released into the public domain.

use crate::Execute;
use dogma::prelude::String;
use typed_builder::TypedBuilder;

/// Network protocol fetcher. Consumes a URL input, produces some output.
pub trait Fetcher<T, E>: Execute<T, E> {}

/// Configuration options for [`Fetcher`].
///
/// # Examples
///
/// ```rust
/// use asimov_patterns::FetcherOptions;
///
/// let options = FetcherOptions::builder()
///     .input_url("https://crates.io/robots.txt".to_string())
///     .build();
/// ```
#[derive(Clone, Debug, Default, Eq, Hash, Ord, PartialEq, PartialOrd, TypedBuilder)]
pub struct FetcherOptions {
    pub input_url: String,
}

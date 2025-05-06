// This is free and unencumbered software released into the public domain.

use typed_builder::TypedBuilder;

/// Network protocol fetcher. Consumes a URL input, produces some output.
pub trait Fetcher {}

/// Configuration options for [`Fetcher`].
///
/// # Examples
///
/// ```rust
/// use asimov_patterns::FetcherOptions;
///
/// let options = FetcherOptions::builder().build();
/// ```
#[derive(Clone, Debug, TypedBuilder)]
pub struct FetcherOptions {}

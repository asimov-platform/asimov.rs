// This is free and unencumbered software released into the public domain.

/// Network protocol fetcher. Consumes a URL input, produces some output.
pub trait Fetcher {}

/// Options for [`Fetcher`].
#[derive(Clone, Debug)]
pub struct FetcherOptions {}

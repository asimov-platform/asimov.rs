// This is free and unencumbered software released into the public domain.

use typed_builder::TypedBuilder;

/// RDF dataset converter. Consumes some input, produces RDF output.
pub trait Reader {}

/// Configuration options for [`Reader`].
///
/// # Examples
///
/// ```rust
/// use asimov_patterns::ReaderOptions;
///
/// let options = ReaderOptions::builder().build();
/// ```
#[derive(Clone, Debug, Default, TypedBuilder)]
pub struct ReaderOptions {}

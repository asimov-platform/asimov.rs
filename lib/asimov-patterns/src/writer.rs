// This is free and unencumbered software released into the public domain.

use typed_builder::TypedBuilder;

/// RDF dataset converter. Consumes RDF input, produces some output.
pub trait Writer {}

/// Configuration options for [`Writer`].
///
/// # Examples
///
/// ```rust
/// use asimov_patterns::WriterOptions;
///
/// let options = WriterOptions::builder().build();
/// ```
#[derive(Clone, Debug, Default, Eq, Hash, Ord, PartialEq, PartialOrd, TypedBuilder)]
pub struct WriterOptions {}

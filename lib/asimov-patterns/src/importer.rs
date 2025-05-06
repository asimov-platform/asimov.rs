// This is free and unencumbered software released into the public domain.

use typed_builder::TypedBuilder;

/// RDF dataset importer. Consumes a URL input, produces RDF output.
pub trait Importer {}

/// Configuration options for [`Importer`].
///
/// # Examples
///
/// ```rust
/// use asimov_patterns::ImporterOptions;
///
/// let options = ImporterOptions::builder().build();
/// ```
#[derive(Clone, Debug, Default, TypedBuilder)]
pub struct ImporterOptions {}

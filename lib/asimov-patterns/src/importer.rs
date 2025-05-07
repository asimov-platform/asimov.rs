// This is free and unencumbered software released into the public domain.

use dogma::prelude::String;
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
#[derive(Clone, Debug, Default, Eq, Hash, Ord, PartialEq, PartialOrd, TypedBuilder)]
pub struct ImporterOptions {
    pub input_url: String,
}

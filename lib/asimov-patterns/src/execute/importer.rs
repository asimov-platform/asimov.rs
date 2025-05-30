// This is free and unencumbered software released into the public domain.

use crate::Execute;
use dogma::prelude::String;
use typed_builder::TypedBuilder;

/// RDF dataset importer. Consumes a URL input, produces RDF output.
pub trait Importer<T, E>: Execute<T, E> {}

/// Configuration options for [`Importer`].
///
/// # Examples
///
/// ```rust
/// use asimov_patterns::ImporterOptions;
///
/// let options = ImporterOptions::builder()
///     .input_url("https://crates.io/robots.txt".to_string())
///     .build();
/// ```
#[derive(Clone, Debug, Default, Eq, Hash, Ord, PartialEq, PartialOrd, TypedBuilder)]
pub struct ImporterOptions {
    pub input_url: String,
}

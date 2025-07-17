// This is free and unencumbered software released into the public domain.

use crate::Execute;
use alloc::{string::String, vec::Vec};
use typed_builder::TypedBuilder;

/// RDF dataset importer. Consumes some input, produces RDF output.
///
/// See: https://asimov-specs.github.io/program-patterns/#reader
pub trait Reader<T, E>: Execute<T, E> {}

/// Configuration options for [`Reader`].
///
/// # Examples
///
/// ```rust
/// use asimov_patterns::ReaderOptions;
///
/// let options = ReaderOptions::builder().build();
/// ```
#[derive(Clone, Debug, Default, Eq, Hash, Ord, PartialEq, PartialOrd, TypedBuilder)]
pub struct ReaderOptions {
    /// The input format.
    pub input: Option<String>,

    /// The output format.
    pub output: Option<String>,

    /// Extended nonstandard program options.
    pub other: Vec<String>,
}

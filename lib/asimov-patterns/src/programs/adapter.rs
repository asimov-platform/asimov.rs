// This is free and unencumbered software released into the public domain.

use crate::Execute;
use alloc::{string::String, vec::Vec};
use typed_builder::TypedBuilder;

/// RDF dataset proxy. Consumes SPARQL input, produces RDF output.
///
/// See: https://asimov-specs.github.io/program-patterns/#adapter
pub trait Adapter<T, E>: Execute<T, E> {}

/// Configuration options for [`Adapter`].
///
/// # Examples
///
/// ```rust
/// use asimov_patterns::AdapterOptions;
///
/// let options = AdapterOptions::builder().build();
/// ```
#[derive(Clone, Debug, Default, Eq, Hash, Ord, PartialEq, PartialOrd, TypedBuilder)]
pub struct AdapterOptions {
    /// The output format.
    pub output: Option<String>,

    /// Extended nonstandard adapter options.
    pub other: Vec<String>,
}

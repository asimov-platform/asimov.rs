// This is free and unencumbered software released into the public domain.

use crate::Execute;
use alloc::{string::String, vec::Vec};
use typed_builder::TypedBuilder;

/// RDF dataset exporter. Consumes RDF input, produces some output.
///
/// See: https://asimov-specs.github.io/program-patterns/#writer
pub trait Writer<T, E>: Execute<T, E> {}

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
pub struct WriterOptions {
    /// The input format.
    pub input: Option<String>,

    /// The output format.
    pub output: Option<String>,

    /// Extended nonstandard writer options.
    pub other: Vec<String>,
}

// This is free and unencumbered software released into the public domain.

use crate::Execute;
use alloc::{string::String, vec::Vec};
use typed_builder::TypedBuilder;

/// Graph generator. Takes no input, produces an RDF output.
///
/// See: https://asimov-specs.github.io/program-patterns/#emitter
pub trait Emitter<T, E>: Execute<T, E> {}

/// Configuration options for [`Emitter`].
///
/// # Examples
///
/// ```rust
/// use asimov_patterns::EmitterOptions;
///
/// let options = EmitterOptions::builder().build();
/// ```
#[derive(Clone, Debug, Default, Eq, Hash, Ord, PartialEq, PartialOrd, TypedBuilder)]
pub struct EmitterOptions {
    /// The output format.
    pub output: Option<String>,

    /// Extended nonstandard emitter options.
    pub other: Vec<String>,
}

// This is free and unencumbered software released into the public domain.

use crate::Execute;
use alloc::{string::String, vec::Vec};
use bon::Builder;

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
/// let options = EmitterOptions::builder()
///     .output("jsonld")
///     .build();
/// ```
#[derive(Clone, Debug, Default, Eq, Hash, Ord, PartialEq, PartialOrd, Builder)]
#[builder(on(String, into))]
pub struct EmitterOptions {
    /// The output format.
    pub output: Option<String>,

    /// Extended nonstandard emitter options.
    #[builder(default)]
    pub other: Vec<String>,
}

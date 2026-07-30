// This is free and unencumbered software released into the public domain.

use crate::Execute;
use alloc::{string::String, vec::Vec};
use bon::Builder;

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
/// let options = AdapterOptions::builder()
///     .output("jsonld")
///     .build();
/// ```
#[derive(Clone, Debug, Default, Eq, Hash, Ord, PartialEq, PartialOrd, Builder)]
#[builder(derive(Debug), on(String, into))]
pub struct AdapterOptions {
    /// Extended nonstandard adapter options.
    #[builder(field)]
    pub other: Vec<String>,

    /// The output format.
    pub output: Option<String>,
}

impl<S: adapter_options_builder::State> AdapterOptionsBuilder<S> {
    pub fn other(mut self, flag: impl Into<String>) -> Self {
        self.other.push(flag.into());
        self
    }
}

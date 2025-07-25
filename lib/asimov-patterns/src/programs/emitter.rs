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
#[builder(derive(Debug), on(String, into))]
pub struct EmitterOptions {
    /// Extended nonstandard emitter options.
    #[builder(field)]
    pub other: Vec<String>,

    /// The output format.
    pub output: Option<String>,
}

impl<S: emitter_options_builder::State> EmitterOptionsBuilder<S> {
    pub fn other(mut self, flag: impl Into<String>) -> Self {
        self.other.push(flag.into());
        self
    }

    pub fn maybe_other(mut self, flag: Option<impl Into<String>>) -> Self {
        if let Some(flag) = flag {
            self.other.push(flag.into());
        }
        self
    }
}

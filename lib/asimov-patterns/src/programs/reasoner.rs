// This is free and unencumbered software released into the public domain.

use crate::Execute;
use alloc::{string::String, vec::Vec};
use bon::Builder;

/// RDF dataset entailer. Consumes RDF input, produces entailed RDF output.
///
/// See: https://asimov-specs.github.io/program-patterns/#reasoner
pub trait Reasoner<T, E>: Execute<T, E> {}

/// Configuration options for [`Reasoner`].
///
/// # Examples
///
/// ```rust
/// use asimov_patterns::ReasonerOptions;
///
/// let options = ReasonerOptions::builder()
///     .input("jsonld")
///     .output("jsonld")
///     .build();
/// ```
#[derive(Clone, Debug, Default, Eq, Hash, Ord, PartialEq, PartialOrd, Builder)]
#[builder(derive(Debug), on(String, into))]
pub struct ReasonerOptions {
    /// Extended nonstandard reasoner options.
    #[builder(field)]
    pub other: Vec<String>,

    /// The input format.
    pub input: Option<String>,

    /// The output format.
    pub output: Option<String>,
}

impl<S: reasoner_options_builder::State> ReasonerOptionsBuilder<S> {
    pub fn other(mut self, flag: impl Into<String>) -> Self {
        self.other.push(flag.into());
        self
    }
}

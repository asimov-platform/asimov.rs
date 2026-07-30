// This is free and unencumbered software released into the public domain.

use crate::Execute;
use alloc::{string::String, vec::Vec};
use bon::Builder;

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
/// let options = WriterOptions::builder()
///     .input("jsonld")
///     .output("jsonld")
///     .build();
/// ```
#[derive(Clone, Debug, Default, Eq, Hash, Ord, PartialEq, PartialOrd, Builder)]
#[builder(derive(Debug), on(String, into))]
pub struct WriterOptions {
    /// Extended nonstandard writer options.
    #[builder(field)]
    pub other: Vec<String>,

    /// The input format.
    pub input: Option<String>,

    /// The output format.
    pub output: Option<String>,
}

impl<S: writer_options_builder::State> WriterOptionsBuilder<S> {
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

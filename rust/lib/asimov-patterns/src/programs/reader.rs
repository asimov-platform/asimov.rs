// This is free and unencumbered software released into the public domain.

use crate::Execute;
use alloc::{string::String, vec::Vec};
use bon::Builder;

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
/// let options = ReaderOptions::builder()
///     .input("jsonld")
///     .output("jsonld")
///     .build();
/// ```
#[derive(Clone, Debug, Default, Eq, Hash, Ord, PartialEq, PartialOrd, Builder)]
#[builder(derive(Debug), on(String, into))]
pub struct ReaderOptions {
    /// Extended nonstandard reader options.
    #[builder(field)]
    pub other: Vec<String>,

    /// The input format.
    pub input: Option<String>,

    /// The output format.
    pub output: Option<String>,
}

impl<S: reader_options_builder::State> ReaderOptionsBuilder<S> {
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

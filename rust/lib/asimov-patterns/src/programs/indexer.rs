// This is free and unencumbered software released into the public domain.

use crate::Execute;
use alloc::{string::String, vec::Vec};
use bon::Builder;

/// RDF dataset indexer. Consumes RDF input, maintains a persistent index.
///
/// See: https://asimov-specs.github.io/program-patterns/#indexer
pub trait Indexer<E>: Execute<(), E> {}

/// Configuration options for [`Indexer`].
///
/// # Examples
///
/// ```rust
/// use asimov_patterns::IndexerOptions;
///
/// let options = IndexerOptions::builder()
///     .input("jsonld")
///     .build();
/// ```
#[derive(Clone, Debug, Default, Eq, Hash, Ord, PartialEq, PartialOrd, Builder)]
#[builder(derive(Debug), on(String, into))]
pub struct IndexerOptions {
    /// Extended nonstandard indexer options.
    #[builder(field)]
    pub other: Vec<String>,

    /// The input format.
    pub input: Option<String>,
}

impl<S: indexer_options_builder::State> IndexerOptionsBuilder<S> {
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

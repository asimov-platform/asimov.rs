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
/// let options = IndexerOptions::builder().build();
/// ```
#[derive(Clone, Debug, Default, Eq, Hash, Ord, PartialEq, PartialOrd, Builder)]
pub struct IndexerOptions {
    /// The input format.
    pub input: Option<String>,

    /// Extended nonstandard indexer options.
    pub other: Vec<String>,
}

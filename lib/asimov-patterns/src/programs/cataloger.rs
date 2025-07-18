// This is free and unencumbered software released into the public domain.

use crate::Execute;
use alloc::{string::String, vec::Vec};
use bon::Builder;

/// Graph iterator. Takes a URL input, produces RDF output.
///
/// See: https://asimov-specs.github.io/program-patterns/#cataloger
pub trait Cataloger<T, E>: Execute<T, E> {}

/// Configuration options for [`Cataloger`].
///
/// # Examples
///
/// ```rust
/// use asimov_patterns::CatalogerOptions;
///
/// let options = CatalogerOptions::builder().build();
/// ```
#[derive(Clone, Debug, Default, Eq, Hash, Ord, PartialEq, PartialOrd, Builder)]
pub struct CatalogerOptions {
    /// The maximum number of outputs.
    pub limit: Option<usize>,

    /// The output format.
    pub output: Option<String>,

    /// Extended nonstandard cataloger options.
    pub other: Vec<String>,
}

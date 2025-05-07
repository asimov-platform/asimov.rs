// This is free and unencumbered software released into the public domain.

use crate::Execute;
use typed_builder::TypedBuilder;

/// RDF dataset reasoner. Consumes RDF input, produces entailed RDF output.
pub trait Reasoner<T, E>: Execute<T, E> {}

/// Configuration options for [`Reasoner`].
///
/// # Examples
///
/// ```rust
/// use asimov_patterns::ReasonerOptions;
///
/// let options = ReasonerOptions::builder().build();
/// ```
#[derive(Clone, Debug, Default, Eq, Hash, Ord, PartialEq, PartialOrd, TypedBuilder)]
pub struct ReasonerOptions {}

// This is free and unencumbered software released into the public domain.

use crate::Execute;
use typed_builder::TypedBuilder;

/// RDF dataset adapter. Consumes SPARQL input, produces RDF output.
pub trait Adapter<T, E>: Execute<T, E> {}

/// Configuration options for [`Adapter`].
///
/// # Examples
///
/// ```rust
/// use asimov_patterns::AdapterOptions;
///
/// let options = AdapterOptions::builder().build();
/// ```
#[derive(Clone, Debug, Default, Eq, Hash, Ord, PartialEq, PartialOrd, TypedBuilder)]
pub struct AdapterOptions {}

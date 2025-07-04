// This is free and unencumbered software released into the public domain.

use crate::Execute;
use typed_builder::TypedBuilder;

/// Namespace registrar. Consumes a URL input, produces RDF output.
pub trait Registrar<T, E>: Execute<T, E> {}

/// Configuration options for [`Registrar`].
///
/// # Examples
///
/// ```rust
/// use asimov_patterns::RegistrarOptions;
///
/// let options = RegistrarOptions::builder().build();
/// ```
#[derive(Clone, Debug, Default, Eq, Hash, Ord, PartialEq, PartialOrd, TypedBuilder)]
pub struct RegistrarOptions {}

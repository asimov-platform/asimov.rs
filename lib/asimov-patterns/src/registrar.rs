// This is free and unencumbered software released into the public domain.

/// Namespace registrar. Consumes a URL input, produces RDF output.
pub trait Registrar {}

/// Options for [`Registrar`].
#[derive(Clone, Debug)]
pub struct RegistrarOptions {}

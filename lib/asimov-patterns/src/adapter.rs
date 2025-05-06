// This is free and unencumbered software released into the public domain.

/// RDF dataset adapter. Consumes SPARQL input, produces RDF output.
pub trait Adapter {}

/// Options for [`Adapter`].
#[derive(Clone, Debug)]
pub struct AdapterOptions {}

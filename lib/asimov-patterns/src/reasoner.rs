// This is free and unencumbered software released into the public domain.

/// RDF dataset reasoner. Consumes RDF input, produces entailed RDF output.
pub trait Reasoner {}

/// Options for [`Reasoner`].
#[derive(Clone, Debug)]
pub struct ReasonerOptions {}

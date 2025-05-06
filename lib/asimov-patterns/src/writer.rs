// This is free and unencumbered software released into the public domain.

/// RDF dataset converter. Consumes RDF input, produces some output.
pub trait Writer {}

/// Options for [`Writer`].
#[derive(Clone, Debug)]
pub struct WriterOptions {}

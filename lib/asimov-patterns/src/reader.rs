// This is free and unencumbered software released into the public domain.

/// RDF dataset converter. Consumes some input, produces RDF output.
pub trait Reader {}

/// Options for [`Reader`].
#[derive(Clone, Debug)]
pub struct ReaderOptions {}

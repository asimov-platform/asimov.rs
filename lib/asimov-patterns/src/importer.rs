// This is free and unencumbered software released into the public domain.

/// RDF dataset importer. Consumes a URL input, produces RDF output.
pub trait Importer {}

/// Options for [`Importer`].
#[derive(Clone, Debug)]
pub struct ImporterOptions {}

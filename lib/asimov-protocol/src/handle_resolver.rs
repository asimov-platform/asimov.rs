// This is free and unencumbered software released into the public domain.

use crate::PeerId;
use asimov_id::Handle;
use futures_lite::{Stream, stream};

/// A resolver for ASIMOV handles (e.g., "Ⓐjhacker").
pub trait HandleResolver {
    /// Resolves an ASIMOV handle into a set of peer IDs.
    fn resolve_handle(&mut self, _handle: impl Into<Handle>) -> impl Stream<Item = PeerId> {
        stream::empty()
    }
}

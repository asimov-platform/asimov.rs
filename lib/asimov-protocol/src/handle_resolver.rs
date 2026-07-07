// This is free and unencumbered software released into the public domain.

use crate::PeerId;
use alloc::boxed::Box;
use asimov_id::{Handle, Id};
use core::pin::Pin;
use futures_lite::{Stream, stream};

/// A resolver for ASIMOV handles (e.g., "Ⓐjhacker").
pub trait HandleResolver {
    /// Resolves an ASIMOV ID into a set of peer IDs.
    fn resolve_id(&mut self, id: impl Into<Id>) -> impl Stream<Item = PeerId> + Send {
        let output: Pin<Box<dyn Stream<Item = PeerId> + Send>> = match id.into() {
            Id::Handle(handle) => Box::pin(self.resolve_handle(handle)),
            Id::PublicKey(key) => Box::pin(stream::once(key)),
        };
        output
    }

    /// Resolves an ASIMOV handle into a set of peer IDs.
    fn resolve_handle(&mut self, _handle: impl Into<Handle>) -> impl Stream<Item = PeerId> + Send {
        stream::empty()
    }
}

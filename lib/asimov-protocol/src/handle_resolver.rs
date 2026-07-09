// This is free and unencumbered software released into the public domain.

use crate::PeerId;
use alloc::boxed::Box;
use asimov_id::{Handle, Id};
use core::pin::Pin;
use futures_lite::{Stream, StreamExt, stream};

/// A resolver for ASIMOV handles (e.g., "Ⓐjhacker").
pub trait HandleResolver {
    type Error: core::fmt::Debug + Send;

    /// Resolves an ASIMOV ID and yields a random known peer ID.
    /// Ignores any erroneous results, sampling from only successful results.
    ///
    /// The default implementation requires the `random` feature.
    fn resolve_random(
        &mut self,
        id: impl Into<Id>,
    ) -> impl Future<Output = Result<Option<PeerId>, Self::Error>> {
        #[cfg(not(feature = "random"))]
        unimplemented!("resolve_random requires the `random` feature");

        #[cfg(feature = "random")]
        async move {
            let mut stream = Box::pin(self.resolve_all(id));
            let mut results = alloc::vec::Vec::new();
            while let Some(result) = stream.next().await {
                let Ok(result) = result else {
                    continue; // ignore errors silently
                };
                results.push(result);
            }
            let index = fastrand::usize(..results.len());
            Ok(results.get(index).copied())
        }
    }

    /// Resolves an ASIMOV ID and yields only the first known peer ID.
    /// Ignores any initial erroneous results, returning the first successful result.
    fn resolve_first(
        &mut self,
        id: impl Into<Id>,
    ) -> impl Future<Output = Result<Option<PeerId>, Self::Error>> {
        async move {
            let mut stream = Box::pin(self.resolve_all(id));
            while let Some(result) = stream.next().await {
                let Ok(result) = result else {
                    continue; // ignore errors silently
                };
                return Ok(Some(result));
            }
            Ok(None)
        }
    }

    /// Resolves an ASIMOV ID into a stream of all known peer IDs.
    fn resolve_all(
        &mut self,
        id: impl Into<Id>,
    ) -> impl Stream<Item = Result<PeerId, Self::Error>> + Send {
        let output: Pin<Box<dyn Stream<Item = Result<PeerId, Self::Error>> + Send>> =
            match id.into() {
                Id::Handle(handle) => Box::pin(self.resolve_handle(handle)),
                Id::PublicKey(key) => Box::pin(stream::once(Ok(key))),
            };
        output
    }

    /// Resolves an ASIMOV handle into a stream of all known peer IDs.
    ///
    /// This is the only method that trait implementors must provide.
    fn resolve_handle(
        &mut self,
        _handle: impl Into<Handle>,
    ) -> impl Stream<Item = Result<PeerId, Self::Error>> + Send {
        Box::pin(stream::empty())
    }
}

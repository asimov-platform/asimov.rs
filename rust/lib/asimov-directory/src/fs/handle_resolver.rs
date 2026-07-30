// This is free and unencumbered software released into the public domain.

use super::{ConfigProfile, StateDirectory};
use asimov_protocol::{CsvHandleResolver, Handle, PeerId, Stream};
use std::io::Result;

pub use asimov_protocol::ResolveHandle;

/// A resolver from ASIMOV handles to peer IDs.
///
/// Reads the CSV file at `~/.asimov/configs/<profile>/peers.csv`.
/// The format of the CSV file is simply `handle,peer_id`.
/// The records should be sorted for efficient resolution.
/// The file is read line-by-line, so it can be very large.
pub struct HandleResolver(#[allow(unused)] CsvHandleResolver);

impl HandleResolver {
    pub async fn default() -> Result<Self> {
        let configs = StateDirectory::home()?.configs()?;
        let profile = configs.default_profile()?;
        Self::open(&profile).await
    }

    pub async fn open(profile: &ConfigProfile) -> Result<Self> {
        let path = profile.join("peers.csv");
        Ok(Self(CsvHandleResolver::open(path.as_str()).await?))
    }

    pub fn handles(&mut self) -> impl Stream<Item = Result<Handle>> + Send {
        self.0.handles()
    }

    pub fn records(&mut self) -> impl Stream<Item = Result<(Handle, PeerId)>> + Send {
        self.0.records()
    }
}

impl ResolveHandle for HandleResolver {
    type Error = std::io::Error;

    fn resolve_handle(
        &mut self,
        handle: impl Into<Handle>,
    ) -> impl Stream<Item = Result<PeerId>> + Send {
        self.0.resolve_handle(handle)
    }
}

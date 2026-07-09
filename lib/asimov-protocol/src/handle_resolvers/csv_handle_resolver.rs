// This is free and unencumbered software released into the public domain.

use crate::{PeerId, ResolveHandle};
use alloc::string::String;
use asimov_id::Handle;
use core::str::FromStr;
use csv_async::{AsyncReader, AsyncReaderBuilder, StringRecord};
use futures_lite::{Stream, stream};
use iroh::EndpointId;
use std::io::{Error, Result};
use tokio::fs::File;
use tokio::io::AsyncReadExt;

/// A CSV file resolver for ASIMOV handles.
///
/// The format of the CSV file is simply `handle,peer_id`.
/// The records should be sorted for efficient resolution.
/// The file is read line-by-line, so it can be very large.
pub struct CsvHandleResolver(AsyncReader<File>);

impl CsvHandleResolver {
    /// Opens a CSV file for resolving ASIMOV handles.
    pub async fn open(path: &str) -> std::io::Result<Self> {
        let file = File::open(path).await?;
        let reader = AsyncReaderBuilder::new()
            .has_headers(false)
            .create_reader(file);
        Ok(Self::from(reader))
    }
}

impl From<AsyncReader<File>> for CsvHandleResolver {
    fn from(reader: AsyncReader<File>) -> Self {
        Self(reader)
    }
}

impl ResolveHandle for CsvHandleResolver {
    type Error = std::io::Error;

    /// Resolves a handle into a set of endpoint IDs.
    fn resolve_handle(
        &mut self,
        handle: impl Into<Handle>,
    ) -> impl Stream<Item = Result<PeerId>> + Send {
        let handle = handle.into().into_string();
        async_stream::stream! {
            let mut record = StringRecord::new();
            while let Ok(true) = self.0.read_record(&mut record).await {
                let Some(record_handle) = record.get(0) else {
                    continue; // skip invalid records
                };
                if record_handle != &handle {
                    continue; // skip records that don't match
                }
                let Some(record_peer_id) = record.get(1) else {
                    continue; // skip invalid records
                };
                let Ok(peer_id) = PeerId::from_str(record_peer_id) else {
                    continue; // skip invalid peer IDs
                };
                yield Ok(peer_id);
            }
        }
    }
}

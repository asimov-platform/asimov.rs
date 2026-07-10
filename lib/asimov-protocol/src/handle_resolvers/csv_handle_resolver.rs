// This is free and unencumbered software released into the public domain.

use crate::{PeerId, ResolveHandle};
use alloc::string::String;
use asimov_id::Handle;
use core::str::FromStr;
use csv_async::{AsyncReader, AsyncReaderBuilder, StringRecord};
use futures_lite::{Stream, StreamExt, pin, stream};
use iroh::EndpointId;
use std::io::{Error, Result};
use tokio::fs::File;
use tokio::io::AsyncReadExt;

#[cfg(not(feature = "std"))]
use alloc::collections::BTreeSet as Set;

#[cfg(feature = "std")]
use std::collections::HashSet as Set;

/// A CSV file resolver from ASIMOV handles to peer IDs.
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

    pub fn handles(&mut self) -> impl Stream<Item = Result<Handle>> + Send {
        async_stream::stream! {
            let mut handles = Set::new();
            let records = self.records();
            pin!(records);
            while let Some(record) = records.next().await {
                let (handle, _) = record?;
                if handles.contains(&handle) {
                    continue; // skip duplicate handles
                }
                handles.insert(handle.clone());
                yield Ok(handle);
            }
        }
    }

    pub fn records(&mut self) -> impl Stream<Item = Result<(Handle, PeerId)>> + Send {
        async_stream::stream! {
            self.0.rewind().await?;
            let mut record = StringRecord::new();
            while let Ok(true) = self.0.read_record(&mut record).await {
                let Some(record_handle) = record.get(0) else {
                    continue; // skip invalid records
                };
                let Some(record_endpoint) = record.get(1) else {
                    continue; // skip invalid records
                };
                let Ok(handle) = record_handle.parse::<Handle>() else {
                    continue; // skip invalid handles
                };
                let Ok(endpoint) = record_endpoint.parse::<PeerId>() else {
                    continue; // skip invalid peer IDs
                };
                yield Ok((handle, endpoint));
            }
        }
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
        let handle = handle.into();
        async_stream::stream! {
            let mut endpoints = Set::new();
            let records = self.records();
            pin!(records);
            while let Some(record) = records.next().await {
                let (record_handle, record_endpoint) = record?;
                if record_handle != handle {
                    continue; // skip records that don't match
                }
                if endpoints.contains(&record_endpoint) {
                    continue; // skip duplicate endpoints
                }
                endpoints.insert(record_endpoint.clone());
                yield Ok(record_endpoint);
            }
        }
    }
}

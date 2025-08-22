// This is free and unencumbered software released into the public domain.

#[cfg(feature = "storage-fs")]
mod fs;
#[cfg(feature = "storage-fs")]
pub use fs::*;

use jiff::Timestamp;
use std::{io::Result, string::String, vec::Vec};

use crate::Snapshot;

pub trait Storage {
    fn save(&self, snapshot: &Snapshot) -> Result<()> {
        self.save_timestamp(snapshot)?;
        match self.current_version(&snapshot.url) {
            Ok(current) if snapshot.start_timestamp > current => {
                self.set_current_version(&snapshot.url, snapshot.start_timestamp)
            },
            Ok(_) => Ok(()),

            Err(err) if err.kind() == std::io::ErrorKind::NotFound => {
                self.set_current_version(&snapshot.url, snapshot.start_timestamp)
            },
            Err(err) => Err(err),
        }
    }

    fn save_timestamp(&self, _snapshot: &Snapshot) -> Result<()>;

    fn read(&self, _url: impl AsRef<str>, _timestamp: Timestamp) -> Result<Snapshot>;

    fn read_current(&self, url: impl AsRef<str>) -> Result<Snapshot> {
        let ts = self.current_version(&url)?;
        self.read(&url, ts)
    }

    fn set_current_version(&self, _url: impl AsRef<str>, _timestamp: Timestamp) -> Result<()>;

    fn current_version(&self, _url: impl AsRef<str>) -> Result<Timestamp>;

    fn list_urls(&self) -> Result<Vec<(String, Timestamp)>>;

    fn list_snapshots(&self, _url: impl AsRef<str>) -> Result<Vec<Timestamp>>;

    fn delete(&self, _url: impl AsRef<str>, _timestamp: Timestamp) -> Result<()>;
}

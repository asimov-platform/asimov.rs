// This is free and unencumbered software released into the public domain.

#[cfg(feature = "storage-fs")]
mod fs;
#[cfg(feature = "storage-fs")]
pub use fs::*;

use jiff::Timestamp;
use std::{io::Result, string::String, vec::Vec};

pub trait Storage {
    fn save(
        &self,
        url: impl AsRef<str>,
        timestamp: Timestamp,
        data: impl AsRef<[u8]>,
    ) -> Result<()> {
        self.save_timestamp(&url, timestamp, data)?;
        match self.current_version(&url) {
            Ok(current) if timestamp > current => self.set_current_version(&url, timestamp),
            Ok(_) => Ok(()),

            Err(err) if err.kind() == std::io::ErrorKind::NotFound => {
                self.set_current_version(&url, timestamp)
            },
            Err(err) => Err(err),
        }
    }

    fn save_timestamp(
        &self,
        _url: impl AsRef<str>,
        _timestamp: Timestamp,
        _data: impl AsRef<[u8]>,
    ) -> Result<()>;

    fn read(&self, _url: impl AsRef<str>, _timestamp: Timestamp) -> Result<Vec<u8>>;

    fn read_current(&self, url: impl AsRef<str>) -> Result<Vec<u8>> {
        let ts = self.current_version(&url)?;
        self.read(&url, ts)
    }

    fn set_current_version(&self, _url: impl AsRef<str>, _timestamp: Timestamp) -> Result<()>;

    fn current_version(&self, _url: impl AsRef<str>) -> Result<Timestamp>;

    fn list_urls(&self) -> Result<Vec<(String, Timestamp)>>;

    fn list_snapshots(&self, _url: impl AsRef<str>) -> Result<Vec<Timestamp>>;

    fn delete(&self, _url: impl AsRef<str>, _timestamp: Timestamp) -> Result<()>;
}

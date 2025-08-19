// This is free and unencumbered software released into the public domain.

#![no_std]

#[cfg(feature = "std")]
extern crate std;

#[cfg(feature = "std")]
mod snapshot;
#[cfg(feature = "std")]
pub use snapshot::*;

#[cfg(feature = "std")]
pub mod storage;

#[derive(Clone, Debug, bon::Builder)]
pub struct Snapshot {
    #[builder(into)]
    pub url: std::string::String,
    #[builder(into)]
    pub data: std::vec::Vec<u8>,
    pub start_timestamp: jiff::Timestamp,
    pub end_timestamp: Option<jiff::Timestamp>,
}

// This is free and unencumbered software released into the public domain.

//#![no_std]
#![forbid(unsafe_code)]

extern crate alloc;

#[cfg(feature = "std")]
extern crate std;

pub use tokio::{net::ToSocketAddrs, task::JoinHandle};
pub use tokio_util::sync::CancellationToken;

#[cfg(feature = "http")]
pub mod http;

#[cfg(feature = "persistence")]
pub mod persistence;

#[cfg(feature = "mdns")]
pub mod mdns;

#[cfg(feature = "ssdp")]
pub mod ssdp;

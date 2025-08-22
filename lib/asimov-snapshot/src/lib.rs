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

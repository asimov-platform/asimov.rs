// This is free and unencumbered software released into the public domain.

#![no_std]
#![forbid(unsafe_code)]

extern crate alloc;

#[cfg(feature = "std")]
extern crate std;

#[cfg(feature = "std")]
mod snapshot;
#[cfg(feature = "std")]
pub use snapshot::*;

#[cfg(feature = "std")]
pub mod storage;

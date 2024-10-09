// This is free and unencumbered software released into the public domain.

#![no_std]
#![deny(unsafe_code)]

extern crate alloc;

#[cfg(all(feature = "serde", feature = "yaml"))]
mod yaml;
#[cfg(all(feature = "serde", feature = "yaml"))]
pub use yaml::*;

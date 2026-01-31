// This is free and unencumbered software released into the public domain.

#![no_std]
#![forbid(unsafe_code)]

extern crate alloc;

#[cfg(feature = "std")]
extern crate std;

pub use asimov_module::{InstalledModuleManifest, ModuleManifest};

#[cfg(feature = "std")]
pub mod registry;
#[cfg(feature = "std")]
pub use registry::*;

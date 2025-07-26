// This is free and unencumbered software released into the public domain.

#![no_std]
#![forbid(unsafe_code)]

extern crate alloc;

#[cfg(feature = "std")]
extern crate std;

pub use asimov_module::{InstalledModuleManifest, ModuleManifest};
pub use dogma::prelude;

#[cfg(feature = "std")]
mod installer;
#[cfg(feature = "std")]
pub use installer::*;

// This is free and unencumbered software released into the public domain.

#![no_std]
#![forbid(unsafe_code)]

extern crate alloc;

#[cfg(feature = "std")]
extern crate std;

mod config_directory;
pub use config_directory::*;

mod config_profile;
pub use config_profile::*;

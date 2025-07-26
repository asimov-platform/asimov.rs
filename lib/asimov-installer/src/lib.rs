// This is free and unencumbered software released into the public domain.

#![no_std]
#![forbid(unsafe_code)]

extern crate alloc;

#[cfg(feature = "std")]
extern crate std;

pub use dogma::prelude;

mod installer;
pub use installer::*;

mod models;
pub use models::*;

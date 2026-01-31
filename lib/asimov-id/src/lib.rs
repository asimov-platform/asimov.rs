// This is free and unencumbered software released into the public domain.

#![no_std]
#![forbid(unsafe_code)]

extern crate alloc;

#[cfg(feature = "std")]
extern crate std;

mod id;
pub use id::*;

mod id_error;
pub use id_error::*;

mod key;
pub use key::*;

mod key_error;
pub use key_error::*;

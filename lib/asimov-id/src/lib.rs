// This is free and unencumbered software released into the public domain.

#![no_std]
#![forbid(unsafe_code)]
#![allow(unused_imports)]
#![cfg_attr(docsrs, feature(doc_cfg))]

extern crate alloc;

#[cfg(feature = "std")]
extern crate std;

mod handle;
pub use handle::*;

mod handle_error;
pub use handle_error::*;

mod key;
pub use key::*;

mod key_error;
pub use key_error::*;

mod public_key;
pub use public_key::*;

mod secret_key;
pub use secret_key::*;

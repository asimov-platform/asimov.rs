// This is free and unencumbered software released into the public domain.

#![no_std]
#![forbid(unsafe_code)]

extern crate alloc;

#[cfg(feature = "std")]
extern crate std;

mod prompt;
pub use prompt::*;

mod prompt_message;
pub use prompt_message::*;

mod prompt_role;
pub use prompt_role::*;

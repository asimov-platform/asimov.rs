// This is free and unencumbered software released into the public domain.

#![no_std]
#![forbid(unsafe_code)]

extern crate alloc;

#[cfg(feature = "std")]
extern crate std;

#[cfg(feature = "fs")]
pub mod fs;

pub use asimov_config::{ConfigDirectory, ConfigProfile};

mod module_directory;
pub use module_directory::*;

mod module_iterators;
pub use module_iterators::*;

mod program_directory;
pub use program_directory::*;

mod state_directory;
pub use state_directory::*;

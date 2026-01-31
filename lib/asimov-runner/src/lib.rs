// This is free and unencumbered software released into the public domain.

#![no_std]
#![forbid(unsafe_code)]

extern crate alloc;

#[cfg(feature = "std")]
extern crate std;

pub use asimov_patterns::Execute;
pub use clientele::SysexitsError;
pub use tokio::process::Command;

#[cfg(feature = "std")]
pub mod executor;
#[cfg(feature = "std")]
pub use executor::*;

#[cfg(feature = "std")]
pub mod executor_error;
#[cfg(feature = "std")]
pub use executor_error::*;

pub mod input;
pub use input::*;

pub mod output;
pub use output::*;

pub mod pipeline;
pub use pipeline::*;

#[cfg(feature = "std")]
pub mod programs;
#[cfg(feature = "std")]
pub use programs::*;

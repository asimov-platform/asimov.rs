// This is free and unencumbered software released into the public domain.

#![no_std]
#![forbid(unsafe_code)]

extern crate alloc;

pub use dogma::prelude;

#[cfg(feature = "cli")]
pub use clientele::{args_os, dotenv, exit, SysexitsError, SysexitsResult};

#[cfg(feature = "std")]
pub use getenv;

pub use secrecy;

#[cfg(feature = "tracing")]
pub use tracing;

#[cfg(feature = "tracing")]
pub use tracing_subscriber;

pub mod resolve;

pub mod models;

// This is free and unencumbered software released into the public domain.

#![no_std]
#![forbid(unsafe_code)]

pub use dogma::prelude;

#[cfg(feature = "cli")]
pub use clientele::{SysexitsError, SysexitsResult, args_os, dotenv, exit};

#[cfg(feature = "std")]
pub use getenv;

pub use secrecy;

#[cfg(feature = "tracing")]
pub use tracing;

#[cfg(feature = "tracing")]
pub use tracing_subscriber;

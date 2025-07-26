// This is free and unencumbered software released into the public domain.

#![no_std]
#![forbid(unsafe_code)]

extern crate alloc;

#[cfg(feature = "std")]
extern crate std;

pub use dogma::prelude;

#[cfg(feature = "cli")]
pub use clientele::{SysexitsError, SysexitsResult, args_os, dotenv, exit};

#[cfg(feature = "std")]
pub use getenv;

pub use secrecy;

#[cfg(not(feature = "tracing"))]
pub mod tracing;

#[cfg(feature = "tracing")]
pub use tracing;

#[cfg(feature = "tracing")]
pub use tracing_subscriber;

#[cfg(all(feature = "std", feature = "cli", feature = "tracing"))]
pub fn init_tracing_subscriber(
    options: &clientele::StandardOptions,
) -> Result<(), alloc::boxed::Box<(dyn core::error::Error + Send + Sync + 'static)>> {
    extern crate std;
    tracing_subscriber::fmt()
        .with_writer(std::io::stderr)
        .with_max_level(options)
        .with_level(options.debug || options.verbose > 0)
        .with_target(options.debug)
        .with_file(false)
        .without_time()
        .try_init()
}

mod models;
pub use models::*;

pub mod resolve;

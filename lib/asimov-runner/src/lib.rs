// This is free and unencumbered software released into the public domain.

//! ```rust
//! # use asimov_runner::*;
//! ```

#![allow(unused_imports)]

pub use asimov_patterns::Execute;
pub use clientele::SysexitsError;
pub use tokio::process::Command;

pub mod executor;
pub use executor::*;

pub mod executor_error;
pub use executor_error::*;

pub mod input;
pub use input::*;

pub mod output;
pub use output::*;

pub mod programs;
pub use programs::*;

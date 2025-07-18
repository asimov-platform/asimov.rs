// This is free and unencumbered software released into the public domain.

//! ```rust
//! # use asimov_runner::*;
//! ```

#![allow(unused_imports)]

pub use asimov_patterns::Execute;
pub use clientele::SysexitsError;
pub use tokio::process::Command;

pub mod programs;
pub use programs::*;

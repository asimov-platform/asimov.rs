// This is free and unencumbered software released into the public domain.

//! ```rust
//! # use asimov_patterns::*;
//! ```

#![no_std]
#![forbid(unsafe_code)]

extern crate alloc;

pub mod execute;
pub use execute::*;

pub mod programs;
pub use programs::*;

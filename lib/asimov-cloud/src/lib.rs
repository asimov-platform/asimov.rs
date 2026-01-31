// This is free and unencumbered software released into the public domain.

#![no_std]
#![forbid(unsafe_code)]

extern crate alloc;

#[cfg(feature = "std")]
extern crate std;

pub use asimov_credit as credit;
pub use asimov_id as id;
pub use asimov_kb as kb;
pub use asimov_nexus as nexus;

pub mod account;
pub mod error;

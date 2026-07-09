// This is free and unencumbered software released into the public domain.

#![allow(unused_imports)]

#[cfg(all(feature = "csv", feature = "std"))]
mod csv_handle_resolver;
#[cfg(all(feature = "csv", feature = "std"))]
pub use csv_handle_resolver::*;

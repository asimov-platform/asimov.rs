// This is free and unencumbered software released into the public domain.

//! Dummy implementations of `tracing` macros for when tracing is disabled.
//!
//! These macros provide a zero-cost abstraction when tracing is not enabled,
//! allowing code to compile without changes and without runtime overhead.

/// No-op `debug!` macro
#[macro_export]
macro_rules! debug {
    ($($arg:tt)*) => {
        ()
    };
}

/// No-op `error!` macro
#[macro_export]
macro_rules! error {
    ($($arg:tt)*) => {
        ()
    };
}

/// No-op `info!` macro
#[macro_export]
macro_rules! info {
    ($($arg:tt)*) => {
        ()
    };
}

/// No-op `trace!` macro
#[macro_export]
macro_rules! trace {
    ($($arg:tt)*) => {
        ()
    };
}

/// No-op `warn!` macro
#[macro_export]
macro_rules! warn {
    ($($arg:tt)*) => {
        ()
    };
}

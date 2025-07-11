// This is free and unencumbered software released into the public domain.

extern crate alloc;

#[allow(unused)]
pub use alloc::{
    borrow::Cow,
    boxed::Box,
    format,
    string::{FromUtf8Error, FromUtf16Error, String, ToString},
    vec,
    vec::Vec,
};

#[allow(unused)]
pub use core::{
    convert::TryFrom,
    ffi::{FromBytesWithNulError, c_int},
    fmt,
    marker::PhantomData,
    num::{ParseFloatError, ParseIntError},
    ops::Range,
    ptr::{null, null_mut},
    result::Result,
    str::Utf8Error,
    sync::atomic::{AtomicBool, Ordering},
    time::Duration,
};

// This is free and unencumbered software released into the public domain.

#![no_std]
#![allow(non_camel_case_types)]
#![allow(non_upper_case_globals)]

mod util;

extern crate alloc;

#[macro_use]
extern crate num_derive;

#[cfg(feature = "std")]
extern crate std;

use crate::util::string_to_static_array;
use alloc::borrow::Cow;
use core::{
    ffi::{c_int, CStr},
    fmt::{self, Debug, Display},
    mem::size_of,
    str::Utf8Error,
};
use num_traits::FromPrimitive;

include!("bindgen.rs");
include!("consts.rs");
include!("default.rs");
include!("display.rs");
include!("from.rs");
include!("getters.rs");
include!("try_from.rs");

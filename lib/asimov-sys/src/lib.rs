// This is free and unencumbered software released into the public domain.

#![no_std]
#![allow(non_camel_case_types)]
#![allow(non_upper_case_globals)]

#[macro_use]
extern crate num_derive;

#[cfg(feature = "std")]
extern crate std;

use core::{ffi::c_int, mem::size_of};
use num_traits::FromPrimitive;

include!("bindgen.rs");

pub const ASI_NULL_HANDLE: AsiInstance = 0;

impl TryFrom<c_int> for AsiPortState {
    type Error = ();

    fn try_from(code: c_int) -> Result<Self, Self::Error> {
        FromPrimitive::from_i64(code as _).ok_or(())
    }
}

impl TryFrom<c_int> for AsiPortType {
    type Error = ();

    fn try_from(code: c_int) -> Result<Self, Self::Error> {
        FromPrimitive::from_i64(code as _).ok_or(())
    }
}

impl TryFrom<c_int> for AsiResult {
    type Error = ();

    fn try_from(code: c_int) -> Result<Self, Self::Error> {
        FromPrimitive::from_i64(code as _).ok_or(())
    }
}

#[cfg(feature = "std")]
impl From<std::io::Error> for AsiResult {
    fn from(error: std::io::Error) -> Self {
        use std::io::ErrorKind::*;
        match error.kind() {
            NotFound => Self::ASI_ERROR_PRECONDITION_VIOLATED,
            _ => Self::ASI_ERROR_NOT_IMPLEMENTED,
        }
    }
}

const _: () = assert!(size_of::<AsiPortState>() == 4, "sizeof(AsiPortState) == 4");
const _: () = assert!(size_of::<AsiPortType>() == 4, "sizeof(AsiPortType) == 4");
const _: () = assert!(size_of::<AsiResult>() == 4, "sizeof(AsiResult) == 4");

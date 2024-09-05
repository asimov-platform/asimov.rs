// This is free and unencumbered software released into the public domain.

#![no_std]
#![allow(non_camel_case_types)]
#![allow(non_upper_case_globals)]

#[macro_use]
extern crate num_derive;

use core::ffi::c_int;
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

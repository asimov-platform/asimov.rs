// This is free and unencumbered software released into the public domain.

#![no_std]
#![allow(non_camel_case_types)]
#![allow(non_upper_case_globals)]

#[macro_use]
extern crate num_derive;

#[cfg(feature = "std")]
extern crate std;

use core::{
    ffi::{c_int, CStr},
    fmt::{Debug, Display},
    mem::size_of,
    str::Utf8Error,
};
use num_traits::FromPrimitive;

include!("bindgen.rs");

pub const ASI_NULL_HANDLE: AsiInstance = 0;

const _: () = assert!(size_of::<AsiPortState>() == 4, "sizeof(AsiPortState) == 4");
const _: () = assert!(size_of::<AsiPortType>() == 4, "sizeof(AsiPortType) == 4");
const _: () = assert!(size_of::<AsiResult>() == 4, "sizeof(AsiResult) == 4");

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

impl AsiBlockDefinition {
    pub fn name(&self) -> Result<&str, Utf8Error> {
        unsafe { CStr::from_ptr(self.name.as_ptr()) }.to_str()
    }
}

impl AsiBlockParameter {
    pub fn name(&self) -> Result<&str, Utf8Error> {
        unsafe { CStr::from_ptr(self.name.as_ptr()) }.to_str()
    }
}

impl AsiBlockPort {
    pub fn name(&self) -> Result<&str, Utf8Error> {
        unsafe { CStr::from_ptr(self.name.as_ptr()) }.to_str()
    }
}

impl AsiBlockUsage {
    pub fn name(&self) -> Result<&str, Utf8Error> {
        unsafe { CStr::from_ptr(self.name.as_ptr()) }.to_str()
    }
}

impl AsiFlowConnection {}

impl AsiFlowDefinition {
    pub fn name(&self) -> Result<&str, Utf8Error> {
        unsafe { CStr::from_ptr(self.name.as_ptr()) }.to_str()
    }
}

impl AsiFlowExecution {
    pub fn name(&self) -> Result<&str, Utf8Error> {
        unsafe { CStr::from_ptr(self.name.as_ptr()) }.to_str()
    }
}

impl AsiModelManifest {
    pub fn name(&self) -> Result<&str, Utf8Error> {
        unsafe { CStr::from_ptr(self.name.as_ptr()) }.to_str()
    }
}

impl AsiModuleRegistration {
    pub fn name(&self) -> Result<&str, Utf8Error> {
        unsafe { CStr::from_ptr(self.name.as_ptr()) }.to_str()
    }
}

impl Display for AsiBlockDefinition {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_struct("AsiBlockDefinition")
            .field("name", &self.name().expect("name should be valid UTF-8"))
            .field("input_port_count", &self.input_port_count)
            .field("output_port_count", &self.output_port_count)
            .field("parameter_count", &self.parameter_count)
            .finish()
    }
}

impl Display for AsiBlockParameter {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_struct("AsiBlockParameter")
            .field("name", &self.name().expect("name should be valid UTF-8"))
            .field("default_value", &self.default_value)
            .finish()
    }
}

impl Display for AsiBlockPort {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_struct("AsiBlockPort")
            .field("name", &self.name().expect("name should be valid UTF-8"))
            .field("type", &self.type_)
            .finish()
    }
}

impl Display for AsiBlockUsage {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_struct("AsiBlockUsage")
            .field("name", &self.name().expect("name should be valid UTF-8"))
            .field("type", &self.type_)
            .finish()
    }
}

impl Display for AsiFlowConnection {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_struct("AsiFlowConnection")
            .field("source_block", &self.source_block)
            .field("source_port", &self.source_port)
            .field("target_block", &self.target_block)
            .field("target_port", &self.target_port)
            .finish()
    }
}

impl Display for AsiFlowDefinition {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_struct("AsiFlowDefinition")
            .field("name", &self.name().expect("name should be valid UTF-8"))
            .field("block_count", &self.block_count)
            .finish()
    }
}

impl Display for AsiFlowExecution {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_struct("AsiFlowExecution")
            .field("name", &self.name().expect("name should be valid UTF-8"))
            .field("pid", &self.pid)
            .finish()
    }
}

impl Display for AsiModelManifest {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_struct("AsiModelManifest")
            .field("name", &self.name().expect("name should be valid UTF-8"))
            .field("size", &self.size)
            .finish()
    }
}

impl Display for AsiModuleRegistration {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_struct("AsiModuleRegistration")
            .field("name", &self.name().expect("name should be valid UTF-8"))
            .finish()
    }
}

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
    pub fn new(
        name: &str,
        input_port_count: u32,
        output_port_count: u32,
        parameter_count: u32,
    ) -> Self {
        Self {
            name: string_to_static_array(name),
            input_port_count,
            output_port_count,
            parameter_count,
            ..Default::default()
        }
    }

    pub fn name(&self) -> Result<&str, Utf8Error> {
        unsafe { CStr::from_ptr(self.name.as_ptr()) }.to_str()
    }

    pub fn name_lossy(&self) -> Cow<'_, str> {
        unsafe { CStr::from_ptr(self.name.as_ptr()) }.to_string_lossy()
    }
}

impl AsiBlockParameter {
    pub fn new(name: &str, default_value: &str) -> Self {
        Self {
            name: string_to_static_array(name),
            default_value: string_to_static_array(default_value),
            ..Default::default()
        }
    }

    pub fn name(&self) -> Result<&str, Utf8Error> {
        unsafe { CStr::from_ptr(self.name.as_ptr()) }.to_str()
    }

    pub fn name_lossy(&self) -> Cow<'_, str> {
        unsafe { CStr::from_ptr(self.name.as_ptr()) }.to_string_lossy()
    }
}

impl AsiBlockPort {
    pub fn new(name: &str, r#type: AsiPortType) -> Self {
        Self {
            name: string_to_static_array(name),
            type_: r#type,
            ..Default::default()
        }
    }

    pub fn name(&self) -> Result<&str, Utf8Error> {
        unsafe { CStr::from_ptr(self.name.as_ptr()) }.to_str()
    }

    pub fn name_lossy(&self) -> Cow<'_, str> {
        unsafe { CStr::from_ptr(self.name.as_ptr()) }.to_string_lossy()
    }
}

impl AsiBlockUsage {
    pub fn new(name: &str, r#type: &str) -> Self {
        Self {
            name: string_to_static_array(name),
            type_: string_to_static_array(r#type),
            ..Default::default()
        }
    }

    pub fn name(&self) -> Result<&str, Utf8Error> {
        unsafe { CStr::from_ptr(self.name.as_ptr()) }.to_str()
    }

    pub fn name_lossy(&self) -> Cow<'_, str> {
        unsafe { CStr::from_ptr(self.name.as_ptr()) }.to_string_lossy()
    }
}

impl AsiFlowConnection {
    pub fn new(
        source_block: &str,
        source_port: &str,
        target_block: &str,
        target_port: &str,
    ) -> Self {
        Self {
            source_block: string_to_static_array(source_block),
            source_port: string_to_static_array(source_port),
            target_block: string_to_static_array(target_block),
            target_port: string_to_static_array(target_port),
            ..Default::default()
        }
    }

    // TODO: getters for the string fields
}

impl AsiFlowDefinition {
    pub fn new(name: &str, block_count: u32) -> Self {
        Self {
            name: string_to_static_array(name),
            block_count,
            ..Default::default()
        }
    }

    pub fn name(&self) -> Result<&str, Utf8Error> {
        unsafe { CStr::from_ptr(self.name.as_ptr()) }.to_str()
    }

    pub fn name_lossy(&self) -> Cow<'_, str> {
        unsafe { CStr::from_ptr(self.name.as_ptr()) }.to_string_lossy()
    }
}

impl AsiFlowExecution {
    pub fn new(name: &str, pid: u64) -> Self {
        Self {
            name: string_to_static_array(name),
            pid,
            ..Default::default()
        }
    }

    pub fn name(&self) -> Result<&str, Utf8Error> {
        unsafe { CStr::from_ptr(self.name.as_ptr()) }.to_str()
    }

    pub fn name_lossy(&self) -> Cow<'_, str> {
        unsafe { CStr::from_ptr(self.name.as_ptr()) }.to_string_lossy()
    }
}

impl AsiModelManifest {
    pub fn new(name: &str, size: u64) -> Self {
        Self {
            name: string_to_static_array(name),
            size,
            ..Default::default()
        }
    }

    pub fn name(&self) -> Result<&str, Utf8Error> {
        unsafe { CStr::from_ptr(self.name.as_ptr()) }.to_str()
    }

    pub fn name_lossy(&self) -> Cow<'_, str> {
        unsafe { CStr::from_ptr(self.name.as_ptr()) }.to_string_lossy()
    }
}

impl AsiModuleRegistration {
    pub fn new(name: &str, block_count: u32) -> Self {
        Self {
            name: string_to_static_array(name),
            block_count,
            ..Default::default()
        }
    }

    pub fn name(&self) -> Result<&str, Utf8Error> {
        unsafe { CStr::from_ptr(self.name.as_ptr()) }.to_str()
    }

    pub fn name_lossy(&self) -> Cow<'_, str> {
        unsafe { CStr::from_ptr(self.name.as_ptr()) }.to_string_lossy()
    }
}

impl Display for AsiBlockDefinition {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("AsiBlockDefinition")
            .field("name", &self.name_lossy())
            .field("input_port_count", &self.input_port_count)
            .field("output_port_count", &self.output_port_count)
            .field("parameter_count", &self.parameter_count)
            .finish()
    }
}

impl Display for AsiBlockParameter {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("AsiBlockParameter")
            .field("name", &self.name_lossy())
            .field("default_value", &self.default_value)
            .finish()
    }
}

impl Display for AsiBlockPort {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("AsiBlockPort")
            .field("name", &self.name_lossy())
            .field("type", &self.type_)
            .finish()
    }
}

impl Display for AsiBlockUsage {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("AsiBlockUsage")
            .field("name", &self.name_lossy())
            .field("type", &self.type_)
            .finish()
    }
}

impl Display for AsiFlowConnection {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("AsiFlowConnection")
            .field("source_block", &self.source_block)
            .field("source_port", &self.source_port)
            .field("target_block", &self.target_block)
            .field("target_port", &self.target_port)
            .finish()
    }
}

impl Display for AsiFlowDefinition {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("AsiFlowDefinition")
            .field("name", &self.name_lossy())
            .field("block_count", &self.block_count)
            .finish()
    }
}

impl Display for AsiFlowExecution {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("AsiFlowExecution")
            .field("name", &self.name_lossy())
            .field("pid", &self.pid)
            .finish()
    }
}

impl Display for AsiModelManifest {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("AsiModelManifest")
            .field("name", &self.name_lossy())
            .field("size", &self.size)
            .finish()
    }
}

impl Display for AsiModuleRegistration {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("AsiModuleRegistration")
            .field("name", &self.name_lossy())
            .field("block_count", &self.block_count)
            .finish()
    }
}

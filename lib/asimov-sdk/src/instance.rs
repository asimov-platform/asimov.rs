// This is free and unencumbered software released into the public domain.

use asimov_sys::{asiCreateInstance, asiDestroyInstance, AsiInstance, AsiResult, ASI_NULL_HANDLE};
use core::ptr::null;

#[derive(Debug, Default, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct Instance {
    handle: AsiInstance,
}

impl Instance {
    pub fn new() -> Result<Self, AsiResult> {
        let mut instance = Self {
            handle: ASI_NULL_HANDLE,
        };
        match unsafe { asiCreateInstance(null(), &mut instance.handle) } {
            AsiResult::ASI_SUCCESS => Ok(instance),
            error => Err(error),
        }
    }
}

impl Drop for Instance {
    fn drop(&mut self) {
        match unsafe { asiDestroyInstance(self.handle) } {
            AsiResult::ASI_SUCCESS => (),
            _ => unreachable!(),
        }
    }
}

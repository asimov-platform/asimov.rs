// This is free and unencumbered software released into the public domain.

use crate::prelude::Vec;
use asimov_sys::{
    asiCreateInstance, asiDestroyInstance, asiEnumerateBlocks, asiEnumerateFlows,
    asiEnumerateModels, asiEnumerateModules, AsiBlockDefinition, AsiFlowDefinition, AsiInstance,
    AsiModelManifest, AsiModuleRegistration, AsiResult, ASI_NULL_HANDLE,
};
use core::ptr::{null, null_mut};

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

    #[stability::unstable]
    pub fn blocks(&self) -> Result<Vec<AsiBlockDefinition>, AsiResult> {
        let mut count: u32 = 0;
        match unsafe { asiEnumerateBlocks(self.handle, 0, &mut count, null_mut()) } {
            AsiResult::ASI_SUCCESS => (),
            error => return Err(error),
        };
        let mut buffer = Vec::with_capacity(count as _);
        match unsafe { asiEnumerateBlocks(self.handle, count, &mut count, buffer.as_mut_ptr()) } {
            AsiResult::ASI_SUCCESS => Ok(buffer),
            error => return Err(error),
        }
    }

    #[stability::unstable]
    pub fn flows(&self) -> Result<Vec<AsiFlowDefinition>, AsiResult> {
        let mut count: u32 = 0;
        match unsafe { asiEnumerateFlows(self.handle, 0, &mut count, null_mut()) } {
            AsiResult::ASI_SUCCESS => (),
            error => return Err(error),
        };
        let mut buffer = Vec::with_capacity(count as _);
        match unsafe { asiEnumerateFlows(self.handle, count, &mut count, buffer.as_mut_ptr()) } {
            AsiResult::ASI_SUCCESS => Ok(buffer),
            error => return Err(error),
        }
    }

    #[stability::unstable]
    pub fn models(&self) -> Result<Vec<AsiModelManifest>, AsiResult> {
        let mut count: u32 = 0;
        match unsafe { asiEnumerateModels(self.handle, 0, &mut count, null_mut()) } {
            AsiResult::ASI_SUCCESS => (),
            error => return Err(error),
        };
        let mut buffer = Vec::with_capacity(count as _);
        match unsafe { asiEnumerateModels(self.handle, count, &mut count, buffer.as_mut_ptr()) } {
            AsiResult::ASI_SUCCESS => Ok(buffer),
            error => return Err(error),
        }
    }

    #[stability::unstable]
    pub fn modules(&self) -> Result<Vec<AsiModuleRegistration>, AsiResult> {
        let mut count: u32 = 0;
        match unsafe { asiEnumerateModules(self.handle, 0, &mut count, null_mut()) } {
            AsiResult::ASI_SUCCESS => (),
            error => return Err(error),
        };
        let mut buffer = Vec::with_capacity(count as _);
        match unsafe { asiEnumerateModules(self.handle, count, &mut count, buffer.as_mut_ptr()) } {
            AsiResult::ASI_SUCCESS => Ok(buffer),
            error => return Err(error),
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

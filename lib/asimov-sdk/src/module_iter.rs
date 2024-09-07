// This is free and unencumbered software released into the public domain.

use crate::{
    prelude::{null_mut, vec, Box, Vec},
    Error, ModuleRegistration, Result, StaticModuleRegistration,
};
use asimov_sys::{asiEnumerateModules, AsiInstance, AsiModuleRegistration, AsiResult};

pub(crate) struct ModuleRegistrationIter {
    index: usize,
    elements: Vec<AsiModuleRegistration>,
}

impl TryFrom<AsiInstance> for ModuleRegistrationIter {
    type Error = Error;

    fn try_from(instance: AsiInstance) -> Result<Self> {
        let mut count: u32 = 0;
        match unsafe { asiEnumerateModules(instance, 0, &mut count, null_mut()) } {
            AsiResult::ASI_SUCCESS => (),
            error => return Err(error.try_into().unwrap()),
        };

        let mut buffer = vec![AsiModuleRegistration::default(); count as _];
        match unsafe { asiEnumerateModules(instance, count, &mut count, buffer.as_mut_ptr()) } {
            AsiResult::ASI_SUCCESS => (),
            error => return Err(error.try_into().unwrap()),
        };

        Ok(Self::from(buffer))
    }
}

impl From<Vec<AsiModuleRegistration>> for ModuleRegistrationIter {
    fn from(elements: Vec<AsiModuleRegistration>) -> Self {
        Self { index: 0, elements }
    }
}

impl Iterator for ModuleRegistrationIter {
    type Item = Box<dyn ModuleRegistration>;

    fn next(&mut self) -> Option<Self::Item> {
        if self.index < self.elements.len() {
            let element = self.elements[self.index];
            self.index += 1;
            Some(Box::new(StaticModuleRegistration::new(element)) as _)
        } else {
            None // end of iteration
        }
    }
}

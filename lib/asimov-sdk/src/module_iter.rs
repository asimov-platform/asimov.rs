// This is free and unencumbered software released into the public domain.

use crate::{
    prelude::{Box, Vec},
    Error, ModuleRegistration, Result, StaticModuleRegistration,
};
use asimov_sys::{AsiInstance, AsiModuleRegistration};

pub(crate) struct ModuleRegistrationIter {
    index: usize,
    elements: Vec<AsiModuleRegistration>,
}

impl TryFrom<AsiInstance> for ModuleRegistrationIter {
    type Error = Error;

    fn try_from(_instance: AsiInstance) -> Result<Self> {
        todo!() // TODO
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

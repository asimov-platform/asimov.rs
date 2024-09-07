// This is free and unencumbered software released into the public domain.

use crate::{
    prelude::{null_mut, vec, Box, Vec},
    BlockDefinition, Error, LocalBlockDefinition, Result,
};
use asimov_sys::{asiEnumerateBlocks, AsiBlockDefinition, AsiInstance, AsiResult};

pub(crate) struct BlockDefinitionIter {
    index: usize,
    elements: Vec<AsiBlockDefinition>,
}

impl TryFrom<AsiInstance> for BlockDefinitionIter {
    type Error = Error;

    fn try_from(instance: AsiInstance) -> Result<Self> {
        let mut count: u32 = 0;
        match unsafe { asiEnumerateBlocks(instance, 0, &mut count, null_mut()) } {
            AsiResult::ASI_SUCCESS => (),
            error => return Err(error.try_into().unwrap()),
        };

        let mut buffer = vec![AsiBlockDefinition::default(); count as _];
        match unsafe { asiEnumerateBlocks(instance, count, &mut count, buffer.as_mut_ptr()) } {
            AsiResult::ASI_SUCCESS => (),
            error => return Err(error.try_into().unwrap()),
        };

        Ok(Self::from(buffer))
    }
}

impl From<Vec<AsiBlockDefinition>> for BlockDefinitionIter {
    fn from(elements: Vec<AsiBlockDefinition>) -> Self {
        Self { index: 0, elements }
    }
}

impl Iterator for BlockDefinitionIter {
    type Item = Box<dyn BlockDefinition>;

    fn next(&mut self) -> Option<Self::Item> {
        if self.index < self.elements.len() {
            let element = self.elements[self.index];
            self.index += 1;
            Some(Box::new(LocalBlockDefinition::new(element)) as _)
        } else {
            None // end of iteration
        }
    }
}
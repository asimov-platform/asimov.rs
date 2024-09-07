// This is free and unencumbered software released into the public domain.

use crate::{
    prelude::{Box, Vec},
    BlockDefinition, Error, LocalBlockDefinition, Result,
};
use asimov_sys::{AsiBlockDefinition, AsiInstance};

pub(crate) struct BlockDefinitionIter {
    index: usize,
    elements: Vec<AsiBlockDefinition>,
}

impl TryFrom<AsiInstance> for BlockDefinitionIter {
    type Error = Error;

    fn try_from(_instance: AsiInstance) -> Result<Self> {
        todo!() // TODO
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

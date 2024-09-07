// This is free and unencumbered software released into the public domain.

use crate::{
    flow::{FlowDefinition, LocalFlowDefinition},
    prelude::{null_mut, vec, Box, Vec},
    Error, Result,
};
use asimov_sys::{asiEnumerateFlows, AsiFlowDefinition, AsiInstance, AsiResult};

pub(crate) struct FlowDefinitionIter {
    index: usize,
    elements: Vec<AsiFlowDefinition>,
}

impl TryFrom<AsiInstance> for FlowDefinitionIter {
    type Error = Error;

    fn try_from(instance: AsiInstance) -> Result<Self> {
        let mut count: u32 = 0;
        match unsafe { asiEnumerateFlows(instance, 0, &mut count, null_mut()) } {
            AsiResult::ASI_SUCCESS => (),
            error => return Err(error.try_into().unwrap()),
        };

        let mut buffer = vec![AsiFlowDefinition::default(); count as _];
        match unsafe { asiEnumerateFlows(instance, count, &mut count, buffer.as_mut_ptr()) } {
            AsiResult::ASI_SUCCESS => (),
            error => return Err(error.try_into().unwrap()),
        };

        Ok(Self::from(buffer))
    }
}

impl From<Vec<AsiFlowDefinition>> for FlowDefinitionIter {
    fn from(elements: Vec<AsiFlowDefinition>) -> Self {
        Self { index: 0, elements }
    }
}

impl Iterator for FlowDefinitionIter {
    type Item = Box<dyn FlowDefinition>;

    fn next(&mut self) -> Option<Self::Item> {
        if self.index < self.elements.len() {
            let element = self.elements[self.index];
            self.index += 1;
            Some(Box::new(LocalFlowDefinition::new(element)) as _)
        } else {
            None // end of iteration
        }
    }
}

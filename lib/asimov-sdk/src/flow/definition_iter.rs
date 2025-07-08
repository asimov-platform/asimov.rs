// This is free and unencumbered software released into the public domain.

use super::{FlowDefinition, LocalFlowDefinition};
use crate::{
    Error, Result,
    prelude::{Box, Vec, null_mut, vec},
};
use asimov_sys::{AsiFlowDefinition, AsiInstance, AsiResult, asiEnumerateFlows};

pub(crate) struct FlowDefinitionIter {
    #[allow(unused)]
    instance: AsiInstance,
    index: usize,
    elements: Vec<AsiFlowDefinition>,
}

impl FlowDefinitionIter {
    pub fn new(instance: AsiInstance, elements: Vec<AsiFlowDefinition>) -> Self {
        Self {
            instance,
            index: 0,
            elements,
        }
    }
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

        Ok(Self::new(instance, buffer))
    }
}

impl Iterator for FlowDefinitionIter {
    type Item = Box<dyn FlowDefinition>;

    fn next(&mut self) -> Option<Self::Item> {
        if self.index < self.elements.len() {
            let element = self.elements[self.index];
            self.index += 1;
            Some(Box::new(LocalFlowDefinition::new(self.instance, element)) as _)
        } else {
            None // end of iteration
        }
    }
}

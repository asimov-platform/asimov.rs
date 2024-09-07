// This is free and unencumbered software released into the public domain.

use crate::{
    flow::{FlowDefinition, LocalFlowDefinition},
    prelude::{Box, Vec},
    Error, Result,
};
use asimov_sys::{AsiFlowDefinition, AsiInstance};

pub(crate) struct FlowDefinitionIter {
    index: usize,
    elements: Vec<AsiFlowDefinition>,
}

impl TryFrom<AsiInstance> for FlowDefinitionIter {
    type Error = Error;

    fn try_from(_instance: AsiInstance) -> Result<Self> {
        todo!() // TODO
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

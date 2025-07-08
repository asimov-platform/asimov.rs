// This is free and unencumbered software released into the public domain.

use super::FlowExecution;
use crate::{
    BlockDefinition, BlockUsage, MaybeLabeled, MaybeNamed, Result,
    crates::serde,
    prelude::{Box, Cow, String, Vec, fmt::Debug, null_mut, vec},
};
use asimov_sys::{
    AsiBlockDefinition, AsiBlockUsage, AsiFlowDefinition, AsiFlowExecution, AsiInstance, AsiResult,
    asiEnumerateFlowBlocks, asiEnumerateFlowExecutions,
};

#[stability::unstable]
pub trait FlowDefinition: asimov_core::flow::FlowDefinition {
    fn blocks(&self) -> Result<Vec<BlockUsage>>;
    fn history(&self) -> Result<Vec<FlowExecution>>;
}

#[cfg(feature = "serde")]
impl serde::Serialize for dyn FlowDefinition {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        //let flow_definition = self.as_flow_definition(); // FIXME
        //flow_definition.serialize(serializer);
        use serde::ser::SerializeStruct;
        let field_count = 1 + self.label().is_some() as usize;
        let mut state = serializer.serialize_struct("FlowDefinition", field_count)?;
        state.serialize_field("name", &self.name())?;
        match self.label() {
            Some(label) => state.serialize_field("label", &label)?,
            None => state.skip_field("label")?,
        };
        state.end()
    }
}

#[derive(Debug, Default)]
pub(crate) struct LocalFlowDefinition {
    #[allow(unused)]
    pub(crate) instance: AsiInstance,
    pub(crate) inner: AsiFlowDefinition,
}

impl LocalFlowDefinition {
    pub fn named(instance: AsiInstance, name: &str) -> Self {
        Self::new(instance, AsiFlowDefinition::new(name, 0))
    }

    pub fn new(instance: AsiInstance, inner: AsiFlowDefinition) -> Self {
        Self { instance, inner }
    }
}

impl asimov_core::flow::FlowDefinition for LocalFlowDefinition {}

impl MaybeNamed for LocalFlowDefinition {
    fn name(&self) -> Option<Cow<str>> {
        Some(self.inner.name_lossy())
    }
}

impl MaybeLabeled for LocalFlowDefinition {}

impl FlowDefinition for LocalFlowDefinition {
    fn blocks(&self) -> Result<Vec<BlockUsage>> {
        let mut count: u32 = 0;
        match unsafe {
            asiEnumerateFlowBlocks(self.instance, &self.inner, 0, &mut count, null_mut())
        } {
            AsiResult::ASI_SUCCESS => (),
            error => return Err(error.try_into().unwrap()),
        };

        let mut buffer = vec![AsiBlockUsage::default(); count as _];
        match unsafe {
            asiEnumerateFlowBlocks(
                self.instance,
                &self.inner,
                count,
                &mut count,
                buffer.as_mut_ptr(),
            )
        } {
            AsiResult::ASI_SUCCESS => (),
            error => return Err(error.try_into().unwrap()),
        };

        Ok(buffer.into_iter().map(BlockUsage::from).collect())
    }

    fn history(&self) -> Result<Vec<FlowExecution>> {
        let mut count: u32 = 0;
        match unsafe {
            asiEnumerateFlowExecutions(self.instance, &self.inner, 0, &mut count, null_mut())
        } {
            AsiResult::ASI_SUCCESS => (),
            error => return Err(error.try_into().unwrap()),
        };

        let mut buffer = vec![AsiFlowExecution::default(); count as _];
        match unsafe {
            asiEnumerateFlowExecutions(
                self.instance,
                &self.inner,
                count,
                &mut count,
                buffer.as_mut_ptr(),
            )
        } {
            AsiResult::ASI_SUCCESS => (),
            error => return Err(error.try_into().unwrap()),
        };

        Ok(buffer.into_iter().map(FlowExecution::from).collect())
    }
}

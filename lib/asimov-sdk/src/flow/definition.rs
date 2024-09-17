// This is free and unencumbered software released into the public domain.

use crate::{
    prelude::{fmt::Debug, null_mut, vec, Box, Cow, String, Vec},
    BlockDefinition, BlockUsage, Named, Result,
};
use asimov_sys::{
    asiEnumerateFlowBlocks, AsiBlockDefinition, AsiBlockUsage, AsiFlowDefinition, AsiInstance,
    AsiResult,
};

#[stability::unstable]
pub trait FlowDefinition: Named + Debug {
    fn blocks(&self) -> Result<Vec<BlockUsage>>;
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

impl Named for LocalFlowDefinition {
    fn name(&self) -> Cow<str> {
        self.inner.name_lossy()
    }
}

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
}

// This is free and unencumbered software released into the public domain.

use crate::prelude::{fmt::Debug, Cow, Named, String};
use asimov_sys::AsiFlowDefinition;

#[stability::unstable]
pub trait FlowDefinition: Named + Debug {}

#[derive(Debug, Default)]
pub(crate) struct LocalFlowDefinition {
    pub(crate) inner: AsiFlowDefinition,
}

impl LocalFlowDefinition {
    #[allow(unused)]
    pub fn new(name: &str, block_count: u32) -> Self {
        Self {
            inner: AsiFlowDefinition::new(name, block_count),
        }
    }
}

impl From<AsiFlowDefinition> for LocalFlowDefinition {
    fn from(inner: AsiFlowDefinition) -> Self {
        Self { inner }
    }
}

impl Named for LocalFlowDefinition {
    fn name(&self) -> Cow<str> {
        self.inner.name_lossy()
    }
}

impl FlowDefinition for LocalFlowDefinition {}

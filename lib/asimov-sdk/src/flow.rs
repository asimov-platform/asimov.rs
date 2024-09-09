// This is free and unencumbered software released into the public domain.

pub use protoflow::*;

use crate::prelude::{fmt::Debug, Cow, Named, String};
use asimov_sys::AsiFlowDefinition;

#[stability::unstable]
pub trait FlowDefinition: Named + Debug {}

#[derive(Debug)]
pub(crate) struct LocalFlowDefinition {
    inner: AsiFlowDefinition,
}

impl LocalFlowDefinition {
    pub fn new(inner: AsiFlowDefinition) -> Self {
        Self { inner }
    }
}

impl Named for LocalFlowDefinition {
    fn name(&self) -> Cow<str> {
        self.inner.name_lossy()
    }
}

impl FlowDefinition for LocalFlowDefinition {}

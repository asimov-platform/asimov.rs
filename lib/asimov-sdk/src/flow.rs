// This is free and unencumbered software released into the public domain.

pub use protoflow::*;

use crate::{
    prelude::{fmt::Debug, String},
    Named,
};
use asimov_sys::AsiFlowDefinition;

#[stability::unstable]
pub trait FlowDefinition: Debug + Named {}

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
    fn name(&self) -> String {
        self.inner.name_lossy().into_owned()
    }
}

impl FlowDefinition for LocalFlowDefinition {}

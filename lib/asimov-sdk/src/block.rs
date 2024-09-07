// This is free and unencumbered software released into the public domain.

use crate::{
    flow::{BlockDescriptor, PortDescriptor},
    prelude::{fmt::Debug, vec, String, Vec},
    Named,
};
use asimov_sys::AsiBlockDefinition;

#[stability::unstable]
pub trait BlockDefinition: Debug + Named + BlockDescriptor {}

#[derive(Debug)]
pub(crate) struct LocalBlockDefinition {
    inner: AsiBlockDefinition,
}

impl LocalBlockDefinition {
    pub fn new(inner: AsiBlockDefinition) -> Self {
        Self { inner }
    }
}

impl Named for LocalBlockDefinition {
    fn name(&self) -> String {
        self.inner.name_lossy().into_owned()
    }
}

impl BlockDescriptor for LocalBlockDefinition {
    fn name(&self) -> Option<String> {
        Some(Named::name(self))
    }

    fn label(&self) -> Option<String> {
        None
    }

    fn inputs(&self) -> Vec<PortDescriptor> {
        vec![] // TODO
    }

    fn outputs(&self) -> Vec<PortDescriptor> {
        vec![] // TODO
    }
}

impl BlockDefinition for LocalBlockDefinition {}

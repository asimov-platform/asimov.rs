// This is free and unencumbered software released into the public domain.

use crate::{
    flow::{BlockDescriptor, PortDescriptor},
    prelude::{fmt::Debug, null_mut, vec, Cow, String, Vec},
    MaybeLabeled, MaybeNamed,
};
use asimov_sys::{
    asiEnumerateBlockPorts, AsiBlockDefinition, AsiBlockPort, AsiInstance, AsiResult,
};

pub use asimov_core::block::BlockDefinition;

#[derive(Debug)]
pub(crate) struct LocalBlockDefinition {
    instance: AsiInstance,
    inner: AsiBlockDefinition,
}

impl LocalBlockDefinition {
    pub fn new(instance: AsiInstance, inner: AsiBlockDefinition) -> Self {
        Self { instance, inner }
    }

    fn ports(&self) -> Vec<PortDescriptor> {
        if self.inner.input_port_count == 0 && self.inner.output_port_count == 0 {
            return vec![]; // no ports
        }

        let mut count: u32 = 0;
        match unsafe {
            asiEnumerateBlockPorts(self.instance, &self.inner, 0, &mut count, null_mut())
        } {
            AsiResult::ASI_SUCCESS => (),
            _error => unreachable!(), // TODO
        };

        let mut buffer = vec![AsiBlockPort::default(); count as _];
        match unsafe {
            asiEnumerateBlockPorts(
                self.instance,
                &self.inner,
                count,
                &mut count,
                buffer.as_mut_ptr(),
            )
        } {
            AsiResult::ASI_SUCCESS => (),
            _error => unreachable!(), // TODO
        };

        Vec::new() // TODO: Protoflow 0.2.3
    }
}

impl MaybeNamed for LocalBlockDefinition {
    fn name(&self) -> Option<Cow<str>> {
        Some(self.inner.name_lossy())
    }
}

impl MaybeLabeled for LocalBlockDefinition {
    fn label(&self) -> Option<Cow<str>> {
        None
    }
}

impl BlockDescriptor for LocalBlockDefinition {
    fn inputs(&self) -> Vec<PortDescriptor> {
        if self.inner.input_port_count == 0 {
            return vec![]; // no input ports
        }
        self.ports()
            .iter()
            .filter(|port| port.is_input())
            .cloned()
            .collect::<_>()
    }

    fn outputs(&self) -> Vec<PortDescriptor> {
        if self.inner.output_port_count == 0 {
            return vec![]; // no output ports
        }
        self.ports()
            .iter()
            .filter(|port| port.is_output())
            .cloned()
            .collect::<_>()
    }
}

impl BlockDefinition for LocalBlockDefinition {}

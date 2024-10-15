// This is free and unencumbered software released into the public domain.

use crate::{
    flow::{
        BlockDescriptor, ParameterDescriptor, PortDescriptor, PortDirection, PortID, PortState,
    },
    prelude::{fmt::Debug, null_mut, vec, Cow, String, Vec},
    MaybeLabeled, MaybeNamed,
};
pub use asimov_core::block::BlockDefinition;
use asimov_sys::{
    asiEnumerateBlockParameters, asiEnumerateBlockPorts, AsiBlockDefinition, AsiBlockParameter,
    AsiBlockPort, AsiInstance, AsiPortDirection, AsiResult,
};

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

        buffer
            .into_iter()
            .map(|ffi| PortDescriptor {
                direction: match ffi.direction {
                    AsiPortDirection::ASI_PORT_DIRECTION_INPUT => PortDirection::Input,
                    AsiPortDirection::ASI_PORT_DIRECTION_OUTPUT => PortDirection::Output,
                },
                name: Some(ffi.name_lossy().into_owned()),
                label: None,
                r#type: ffi.type_lossy().map(Cow::into_owned),
                id: PortID::try_from(1).unwrap(), // FIXME
                state: PortState::default(),
            })
            .collect::<_>()
    }
}

impl MaybeNamed for LocalBlockDefinition {
    fn name(&self) -> Option<Cow<str>> {
        Some(self.inner.name_lossy())
    }
}

impl MaybeLabeled for LocalBlockDefinition {}

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

    fn parameters(&self) -> Vec<ParameterDescriptor> {
        if self.inner.parameter_count == 0 {
            return vec![]; // no parameters
        }

        let mut count: u32 = 0;
        match unsafe {
            asiEnumerateBlockParameters(self.instance, &self.inner, 0, &mut count, null_mut())
        } {
            AsiResult::ASI_SUCCESS => (),
            _error => unreachable!(), // TODO
        };

        let mut buffer = vec![AsiBlockParameter::default(); count as _];
        match unsafe {
            asiEnumerateBlockParameters(
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

        buffer
            .into_iter()
            .map(|ffi| ParameterDescriptor {
                name: ffi.name_lossy().into_owned(),
                label: None,
                r#type: ffi.type_lossy().map(Cow::into_owned),
                default_value: ffi.default_value_lossy().map(Cow::into_owned),
            })
            .collect::<_>()
    }
}

impl BlockDefinition for LocalBlockDefinition {}

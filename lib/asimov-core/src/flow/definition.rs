// This is free and unencumbered software released into the public domain.

use crate::{MaybeLabeled, MaybeNamed};
use core::fmt::Debug;

pub trait FlowDefinition: AsFlowDefinition + MaybeNamed + MaybeLabeled + Debug {}

pub trait AsFlowDefinition {
    fn as_flow_definition(&self) -> &dyn FlowDefinition;
}

impl<T: FlowDefinition + Sized> AsFlowDefinition for T {
    fn as_flow_definition(&self) -> &dyn FlowDefinition {
        self
    }
}

#[cfg(feature = "serde")]
impl serde::Serialize for dyn FlowDefinition {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut state = serializer.serialize_struct("FlowDefinition", 2)?;
        state.serialize_field("name", &self.name())?;
        state.serialize_field("label", &self.label())?;
        state.end()
    }
}

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

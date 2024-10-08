// This is free and unencumbered software released into the public domain.

use crate::flow::BlockDescriptor;
use core::fmt::Debug;

pub trait BlockDefinition: BlockDescriptor + Debug {}

#[cfg(feature = "serde")]
impl serde::Serialize for dyn BlockDefinition {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        let block_descriptor = self.as_block_descriptor();
        block_descriptor.serialize(serializer)
    }
}

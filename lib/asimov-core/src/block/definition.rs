// This is free and unencumbered software released into the public domain.

use crate::flow::BlockDescriptor;
use core::fmt::Debug;

pub trait BlockDefinition: BlockDescriptor + Debug {}

// This is free and unencumbered software released into the public domain.

use crate::MaybeNamed;
use core::fmt::Debug;

pub trait FlowDefinition: MaybeNamed + Debug {}

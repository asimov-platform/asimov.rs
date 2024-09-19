// This is free and unencumbered software released into the public domain.

use crate::Named;
use core::fmt::Debug;

pub trait ModelManifest: Named + Debug {}

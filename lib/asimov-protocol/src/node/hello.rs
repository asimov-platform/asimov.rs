// This is free and unencumbered software released into the public domain.

use super::NodeFeatureSet;
use serde::{Deserialize, Serialize};

pub const MINIMUM_VERSION: u16 = 0;
pub const MAXIMUM_VERSION: u16 = 0;
pub const REQUIRED_FEATURES: &[&str] = &[];
pub const SUPPORTED_FEATURES: &[&str] = &[];

#[derive(Clone, Debug, Eq, Serialize, Deserialize, PartialEq)]
pub struct NodeHello {
    pub minimum_version: u16,
    pub maximum_version: u16,
    pub required_features: NodeFeatureSet<'static>,
    pub supported_features: NodeFeatureSet<'static>,
}

impl Default for NodeHello {
    fn default() -> Self {
        Self {
            minimum_version: MINIMUM_VERSION,
            maximum_version: MAXIMUM_VERSION,
            required_features: NodeFeatureSet::Borrowed(REQUIRED_FEATURES),
            supported_features: NodeFeatureSet::Borrowed(SUPPORTED_FEATURES),
        }
    }
}

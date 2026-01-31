// This is free and unencumbered software released into the public domain.

use alloc::{string::String, vec::Vec};
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Default, PartialEq, PartialOrd, Deserialize, Serialize)]
#[cfg_attr(feature = "validator", derive(validator::Validate))]
pub struct SearchRequest<const N: usize = 128> {
    #[serde(rename = "@context", skip_serializing_if = "Option::is_none")]
    pub context: Option<String>,

    #[cfg_attr(feature = "validator", validate(length(min = 128)))]
    pub vector: Vec<f32>,

    #[serde(skip_serializing_if = "Option::is_none")]
    #[cfg_attr(feature = "validator", validate(range(min = 0, max = 1)))]
    pub min_score: Option<f32>,

    #[serde(skip_serializing_if = "Option::is_none")]
    #[cfg_attr(feature = "validator", validate(range(min = 1, max = 10)))]
    pub max_count: Option<u8>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_credits: Option<usize>,
}

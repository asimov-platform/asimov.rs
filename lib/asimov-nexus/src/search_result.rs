// This is free and unencumbered software released into the public domain.

use alloc::string::String;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Default, PartialEq, PartialOrd, Deserialize, Serialize)]
#[serde(default)]
pub struct SearchResult {
    pub subject: String,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub score: Option<f32>,
}

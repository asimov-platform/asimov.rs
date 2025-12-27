// This is free and unencumbered software released into the public domain.

use super::SearchResult;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Default, PartialEq, PartialOrd, Deserialize, Serialize)]
#[serde(default)]
pub struct SearchResponse {
    #[serde(rename = "@context", skip_serializing_if = "Option::is_none")]
    pub context: Option<String>,

    pub count: usize,

    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub items: Vec<SearchResult>,
}

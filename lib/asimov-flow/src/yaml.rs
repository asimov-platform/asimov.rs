// This is free and unencumbered software released into the public domain.

use alloc::{
    borrow::Cow,
    format,
    string::{String, ToString},
    vec::Vec,
};
use asimov_core::{flow::FlowDefinition, MaybeNamed};
use core::str::FromStr;

#[derive(Clone, Debug)]
pub struct YamlFlowDefinition {
    pub inputs: Vec<String>,
}

impl YamlFlowDefinition {}

impl MaybeNamed for YamlFlowDefinition {
    fn name(&self) -> Option<Cow<str>> {
        None // unnamed when parsed from a string
    }
}

impl FlowDefinition for YamlFlowDefinition {}

impl FromStr for YamlFlowDefinition {
    type Err = serde_yml::Error;

    fn from_str(input: &str) -> Result<Self, Self::Err> {
        // TODO: consider using the `yaml-rust2` crate instead
        let inputs: Vec<String> = input
            .trim()
            .split("\n---")
            .map(|doc| {
                if doc.starts_with("---") {
                    doc.trim().to_string()
                } else {
                    format!("--- {}", doc.trim())
                }
            })
            .collect();
        Ok(Self { inputs })
    }
}

//#[derive(Clone, Debug, serde::Deserialize)]
//struct BlockID {
//    id: String,
//}

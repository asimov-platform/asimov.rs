// This is free and unencumbered software released into the public domain.

use std::sync::Arc;

use rmcp::model::Content;
use serde_json::{Map, Value};

use super::server::Error;

pub type ToolCallback =
    Arc<dyn Fn(Option<Map<String, Value>>) -> Result<Vec<Content>, Error> + Send + Sync>;

#[derive(Clone)]
pub struct Tool {
    pub name: String,
    pub description: Option<String>,
    pub input_schema: Arc<Map<String, Value>>,
    pub callback: ToolCallback,
}

impl Tool {
    pub fn new<N, D, F>(name: N, description: Option<D>, callback: F) -> Self
    where
        N: Into<String>,
        D: Into<String>,
        F: Fn() -> Result<Vec<Content>, Error> + Send + Sync + 'static,
    {
        Self {
            name: name.into(),
            description: description.map(Into::into),
            input_schema: Arc::new(Map::new()),
            callback: Arc::new(move |_args| callback()),
        }
    }

    pub fn new_with_args<N, D, F>(
        name: N,
        description: Option<D>,
        input_schema: Map<String, Value>,
        callback: F,
    ) -> Self
    where
        N: Into<String>,
        D: Into<String>,
        F: Fn(Option<Map<String, Value>>) -> Result<Vec<Content>, Error> + Send + Sync + 'static,
    {
        Self {
            name: name.into(),
            description: description.map(Into::into),
            input_schema: Arc::new(input_schema),
            callback: Arc::new(callback),
        }
    }
}

// This is free and unencumbered software released into the public domain.

use std::sync::Arc;

use super::server::Error;
use rmcp::model::{PromptArgument, PromptMessage};
use serde_json::{Map, Value};

pub type PromptCallback =
    Arc<dyn Fn(Option<Map<String, Value>>) -> Result<Vec<PromptMessage>, Error> + Send + Sync>;

#[derive(Clone)]
pub struct Prompt {
    pub name: String,
    pub description: Option<String>,
    pub arguments: Option<Vec<PromptArgument>>,
    pub callback: PromptCallback,
}

impl Prompt {
    pub fn new<S, D, F>(name: S, description: Option<D>, callback: F) -> Self
    where
        S: Into<String>,
        D: Into<String>,
        F: Fn() -> Result<Vec<PromptMessage>, Error> + Send + Sync + 'static,
    {
        Self {
            name: name.into(),
            description: description.map(Into::into),
            arguments: None,
            callback: Arc::new(move |_args| callback()),
        }
    }

    pub fn new_with_args<S, D, F>(
        name: S,
        description: Option<D>,
        arguments: Vec<PromptArgument>,
        callback: F,
    ) -> Self
    where
        S: Into<String>,
        D: Into<String>,
        F: Fn(Option<Map<String, Value>>) -> Result<Vec<PromptMessage>, Error>
            + Send
            + Sync
            + 'static,
    {
        Self {
            name: name.into(),
            description: description.map(Into::into),
            arguments: Some(arguments),
            callback: Arc::new(callback),
        }
    }
}

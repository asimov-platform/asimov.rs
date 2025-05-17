// This is free and unencumbered software released into the public domain.

use std::{collections::BTreeMap, sync::Arc};

use rmcp::model::{
    Annotated, Content, Implementation, Prompt, PromptArgument, PromptMessage, ProtocolVersion,
    RawResource, ResourceContents, ResourceTemplate, ServerCapabilities, Tool,
};
use serde_json::{Map, Value};

use super::provider::Provider;

pub type PromptCallback =
    Arc<dyn Fn(Option<Map<String, Value>>) -> Result<Vec<PromptMessage>, Error> + Send + Sync>;

#[derive(Clone)]
pub struct RegisteredPrompt {
    pub name: String,
    pub description: Option<String>,
    pub arguments: Option<Vec<PromptArgument>>,
    pub callback: PromptCallback,
}

#[derive(Clone, Default)]
pub struct Server {
    prompts: BTreeMap<String, RegisteredPrompt>,
}

impl Server {
    pub fn new() -> Self {
        Server::default()
    }

    /// ```rust
    /// # use asimov_server::http::mcp::{Server, Error};
    /// # use rmcp::model::{
    /// #     PromptArgument, PromptMessage,
    /// #     PromptMessageRole,
    /// # };
    /// # let mut server = Server::new();
    /// // Register a simple prompt with no arguments
    /// server.register_prompt(
    ///     "greeting".to_string(),
    ///     Some("A simple greeting prompt".to_string()),
    ///     None,
    ///     |_args| {
    ///         Ok(vec![PromptMessage::new_text(PromptMessageRole::Assistant, format!("Hello, world!"))])
    ///     }
    /// );
    ///
    /// // Register a prompt with arguments
    /// server.register_prompt(
    ///     "personalized_greeting".to_string(),
    ///     Some("A personalized greeting prompt".to_string()),
    ///     Some(vec![
    ///         PromptArgument {
    ///             name: "name".to_string(),
    ///             description: Some("The name to greet".to_string()),
    ///             required: Some(true),
    ///         }
    ///     ]),
    ///     |args| {
    ///         let args = args.ok_or(Error::MissingArgument("args".to_string()))?;
    ///         let name = args.get("name")
    ///             .and_then(|v| v.as_str())
    ///             .ok_or(Error::MissingArgument("name".to_string()))?;
    ///
    ///         Ok(vec![PromptMessage::new_text(PromptMessageRole::Assistant, format!("Hello, {}!", name))])
    ///     }
    /// );
    /// ```
    pub fn register_prompt<F>(
        &mut self,
        name: String,
        description: Option<String>,
        arguments: Option<Vec<PromptArgument>>,
        generator: F,
    ) where
        F: Fn(Option<Map<String, Value>>) -> Result<Vec<PromptMessage>, Error>
            + Send
            + Sync
            + 'static,
    {
        let reg_prompt = RegisteredPrompt {
            name: name.clone(),
            description,
            arguments,
            callback: Arc::new(generator),
        };
        self.prompts.insert(name, reg_prompt);
    }
}

#[derive(Clone)]
pub enum Error {
    UnknownPrompt,
    MissingArgument(String),
}

#[async_trait::async_trait]
impl Provider for Server {
    type Error = Error;

    fn protocol_version(&self) -> ProtocolVersion {
        ProtocolVersion::V_2025_03_26
    }

    fn capabilities(&self) -> ServerCapabilities {
        ServerCapabilities::builder()
            .enable_prompts()
            .enable_resources()
            .enable_tools()
            .build()
    }

    fn implementation(&self) -> Implementation {
        Implementation {
            name: env!("CARGO_CRATE_NAME").to_owned(),
            version: env!("CARGO_PKG_VERSION").to_owned(),
        }
    }

    async fn list_prompts(
        &self,
        _page: Option<String>,
    ) -> Result<(Vec<Prompt>, Option<String>), Self::Error> {
        let prompts = self
            .prompts
            .iter()
            .map(|(name, prompt)| Prompt {
                name: name.clone(),
                description: prompt.description.clone(),
                arguments: prompt.arguments.clone(),
            })
            .collect();

        Ok((prompts, None))
    }

    async fn get_prompt(
        &self,
        name: String,
        arguments: Option<Map<String, Value>>,
    ) -> Result<(Vec<PromptMessage>, Option<String>), Self::Error> {
        let Some(prompt) = self.prompts.get(&name) else {
            return Err(Error::UnknownPrompt);
        };

        if let Some(args) = &prompt.arguments {
            for arg in args {
                if !arg.required.unwrap_or(false) {
                    continue;
                }
                let Some(arguments) = &arguments else {
                    return Err(Error::MissingArgument(arg.name.clone()));
                };

                let provided_arg = arguments.get(&arg.name);

                if provided_arg.is_none() {
                    return Err(Error::MissingArgument(arg.name.clone()));
                }
            }
        }

        let messages = (prompt.callback)(arguments)?;

        Ok((messages, None))
    }

    async fn list_resources(
        &self,
        _page: Option<String>,
    ) -> Result<(Vec<Annotated<RawResource>>, Option<String>), Self::Error> {
        todo!()
    }

    async fn list_resource_templates(
        &self,
        _page: Option<String>,
    ) -> Result<(Vec<ResourceTemplate>, Option<String>), Self::Error> {
        todo!()
    }

    async fn read_resource(&self, _uri: &str) -> Result<Vec<ResourceContents>, Self::Error> {
        todo!()
    }

    async fn list_tools(
        &self,
        _page: Option<String>,
    ) -> Result<(Vec<Tool>, Option<String>), Self::Error> {
        todo!()
    }

    async fn call_tool(
        &self,
        _name: &str,
        _arguments: Option<Map<String, Value>>,
    ) -> Result<(Vec<Content>, Option<bool>), Self::Error> {
        todo!()
    }
}

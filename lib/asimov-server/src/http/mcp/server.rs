// This is free and unencumbered software released into the public domain.

use std::{borrow::Cow, collections::BTreeMap, sync::Arc};

use rmcp::model::{
    self, Annotated, Content, Implementation, PromptArgument, PromptMessage, ProtocolVersion,
    RawResource, ResourceContents, ServerCapabilities,
};
use serde_json::{Map, Value};

use super::provider::Provider;

pub type PromptCallback =
    Arc<dyn Fn(Option<Map<String, Value>>) -> Result<Vec<PromptMessage>, Error> + Send + Sync>;

pub type ResourceCallback = Arc<dyn Fn() -> Result<Vec<ResourceContents>, Error> + Send + Sync>;

pub type ToolCallback =
    Arc<dyn Fn(Option<Map<String, Value>>) -> Result<Vec<Content>, Error> + Send + Sync>;

#[derive(Clone)]
pub struct Prompt {
    pub name: String,
    pub description: Option<String>,
    pub arguments: Option<Vec<PromptArgument>>,
    pub callback: PromptCallback,
}

#[derive(Clone)]
pub struct Resource {
    pub uri: String,
    pub name: String,
    pub description: Option<String>,
    pub mime_type: Option<String>,
    pub size: Option<u32>,
    pub callback: ResourceCallback,
}

#[derive(Clone)]
pub struct ResourceTemplate {
    pub name: String,
    pub uri_template: String,
    pub description: Option<String>,
    pub mime_type: Option<String>,
}

#[derive(Clone)]
pub struct Tool {
    pub name: String,
    pub description: Option<String>,
    pub input_schema: Arc<Map<String, Value>>,
    pub callback: ToolCallback,
}

#[derive(Clone, Default)]
pub struct Server {
    prompts: BTreeMap<String, Prompt>,
    resources: BTreeMap<String, Resource>,
    resource_templates: BTreeMap<String, ResourceTemplate>,
    tools: BTreeMap<String, Tool>,
}

impl Server {
    pub fn new() -> Self {
        Server::default()
    }

    /// ```rust
    /// # use asimov_server::http::mcp::{Server, Error, Prompt};
    /// # use rmcp::model::{
    /// #     PromptArgument, PromptMessage,
    /// #     PromptMessageRole,
    /// # };
    /// # let mut server = Server::new();
    /// // Register a simple prompt with no arguments
    /// server.register_prompt(
    ///     "greeting".to_string(),
    ///     Prompt{
    ///
    ///     }
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
    pub fn register_prompt<F>(&mut self, name: String, prompt: Prompt) {
        self.prompts.insert(name, prompt);
    }

    /// Register a resource with a callback to generate its contents.
    ///
    /// ```rust
    /// # use asimov_server::http::mcp::{Server, Error};
    /// # use rmcp::model::{PromptArgument, ResourceContents};
    /// # let mut server = Server::new();
    /// // Register a simple resource with no arguments
    /// server.register_resource(
    ///     "resource:example".to_string(),
    ///     "Example Resource".to_string(),
    ///     Some("An example resource".to_string()),
    ///     "text/plain".to_string(),
    ///     Some(24),
    ///     None,
    ///     |_args| {
    ///         Ok(vec![ResourceContents::Text("Example resource content".to_string())])
    ///     }
    /// );
    ///
    /// // Register a resource with arguments
    /// server.register_resource(
    ///     "resource:personalized".to_string(),
    ///     "Personalized Resource".to_string(),
    ///     Some("A personalized resource".to_string()),
    ///     "text/plain".to_string(),
    ///     None,
    ///     Some(vec![
    ///         PromptArgument {
    ///             name: "name".to_string(),
    ///             description: Some("The name to include".to_string()),
    ///             required: Some(true),
    ///         }
    ///     ]),
    ///     |args| {
    ///         let args = args.ok_or(Error::MissingArgument("args".to_string()))?;
    ///         let name = args.get("name")
    ///             .and_then(|v| v.as_str())
    ///             .ok_or(Error::MissingArgument("name".to_string()))?;
    ///
    ///         Ok(vec![ResourceContents::Text(format!("Hello, {}!", name))])
    ///     }
    /// );
    /// ```
    pub fn register_resource<F>(&mut self, uri: String, resource: Resource) {
        self.resources.insert(uri, resource);
    }

    /// Register a resource template.
    ///
    /// ```rust
    /// # use asimov_server::http::mcp::{Server, Error};
    /// # use rmcp::model::{PromptArgument};
    /// # let mut server = Server::new();
    /// server.register_resource_template(
    ///     "template:example".to_string(),
    ///     "Example Template".to_string(),
    ///     Some("A template for creating example resources".to_string()),
    ///     "text/plain".to_string(),
    ///     Some(vec![
    ///         PromptArgument {
    ///             name: "content".to_string(),
    ///             description: Some("The content to include".to_string()),
    ///             required: Some(true),
    ///         }
    ///     ]),
    /// );
    /// ```
    pub fn register_resource_template(&mut self, name: String, template: ResourceTemplate) {
        self.resource_templates.insert(name, template);
    }

    /// Register a tool with a callback to handle tool calls.
    ///
    /// ```rust
    /// # use asimov_server::http::mcp::{Server, Error};
    /// # use rmcp::model::{PromptArgument, Content};
    /// # use serde_json::json;
    /// # let mut server = Server::new();
    /// server.register_tool(
    ///     "example_tool".to_string(),
    ///     Some("An example tool".to_string()),
    ///     Some(vec![
    ///         PromptArgument {
    ///             name: "input".to_string(),
    ///             description: Some("The input to process".to_string()),
    ///             required: Some(true),
    ///         }
    ///     ]),
    ///     |args| {
    ///         let args = args.ok_or(Error::MissingArgument("args".to_string()))?;
    ///         let input = args.get("input")
    ///             .and_then(|v| v.as_str())
    ///             .ok_or(Error::MissingArgument("input".to_string()))?;
    ///
    ///         let result = format!("Processed: {}", input);
    ///         Ok(vec![Content::Text(result)])
    ///     }
    /// );
    /// ```
    pub fn register_tool<F>(&mut self, name: String, tool: Tool) {
        self.tools.insert(name, tool);
    }
}

#[derive(Clone)]
pub enum Error {
    UnknownPrompt,
    UnknownResource,
    UnknownTool,
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
    ) -> Result<(Vec<model::Prompt>, Option<String>), Self::Error> {
        let prompts = self
            .prompts
            .iter()
            .map(|(name, prompt)| model::Prompt {
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
    ) -> Result<(Vec<model::PromptMessage>, Option<String>), Self::Error> {
        let Some(prompt) = self.prompts.get(&name) else {
            return Err(Error::UnknownPrompt);
        };

        let messages = (prompt.callback)(arguments)?;

        Ok((messages, None))
    }

    async fn list_resources(
        &self,
        _page: Option<String>,
    ) -> Result<(Vec<Annotated<RawResource>>, Option<String>), Self::Error> {
        let resources = self
            .resources
            .iter()
            .map(|(name, resource)| Annotated {
                raw: RawResource {
                    name: name.clone(),
                    uri: resource.uri.clone(),
                    description: resource.description.clone(),
                    mime_type: resource.mime_type.clone(),
                    size: resource.size,
                },
                annotations: None,
            })
            .collect();
        Ok((resources, None))
    }

    async fn list_resource_templates(
        &self,
        _page: Option<String>,
    ) -> Result<(Vec<model::ResourceTemplate>, Option<String>), Self::Error> {
        let templates = self
            .resource_templates
            .iter()
            .map(|(name, template)| model::ResourceTemplate {
                raw: model::RawResourceTemplate {
                    name: name.clone(),
                    uri_template: template.uri_template.clone(),
                    description: template.description.clone(),
                    mime_type: template.mime_type.clone(),
                },
                annotations: None,
            })
            .collect();
        Ok((templates, None))
    }

    async fn read_resource(&self, uri: &str) -> Result<Vec<ResourceContents>, Self::Error> {
        let Some(resource) = self.resources.get(uri) else {
            return Err(Error::UnknownResource);
        };

        let contents = (resource.callback)()?;

        Ok(contents)
    }

    async fn list_tools(
        &self,
        _page: Option<String>,
    ) -> Result<(Vec<model::Tool>, Option<String>), Self::Error> {
        let tools = self
            .tools
            .iter()
            .map(|(name, tool)| model::Tool {
                name: Cow::from(name.clone()),
                description: tool.description.clone().map(Cow::from),
                input_schema: tool.input_schema.clone(),
                annotations: None,
            })
            .collect();
        Ok((tools, None))
    }

    async fn call_tool(
        &self,
        name: &str,
        arguments: Option<Map<String, Value>>,
    ) -> Result<(Vec<Content>, Option<bool>), Self::Error> {
        let Some(tool) = self.tools.get(name) else {
            return Err(Error::UnknownTool);
        };

        let contents = (tool.callback)(arguments)?;

        Ok((contents, None))
    }
}

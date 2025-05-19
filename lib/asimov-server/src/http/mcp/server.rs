// This is free and unencumbered software released into the public domain.

use std::{borrow::Cow, collections::BTreeMap};

use rmcp::model::{
    self, Annotated, Content, Implementation, ProtocolVersion, RawResource, ResourceContents,
    ServerCapabilities,
};
use serde_json::{Map, Value};

use super::{
    prompt::Prompt,
    provider::Provider,
    resource::{Resource, ResourceTemplate},
    tool::Tool,
};

#[derive(Clone, Debug)]
pub enum Error {
    UnknownPrompt,
    UnknownResource,
    UnknownTool,
    MissingArgument(String),
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

    /// Register a prompt with a callback to generate its messages.
    /// ```rust
    /// # use asimov_server::http::mcp::{Server, Error, Prompt};
    /// # use rmcp::model::{
    /// #     PromptArgument, PromptMessage,
    /// #     PromptMessageRole,
    /// # };
    /// # use serde_json::Value;
    /// # let mut server = Server::new();
    /// // Register a simple prompt with no arguments
    /// let prompt = Prompt::new(
    ///     "greeting",
    ///     Some("A simple greeting prompt"),
    ///     || {
    ///         Ok(vec![PromptMessage::new_text(PromptMessageRole::Assistant, format!("Hello, world!"))])
    ///     });
    /// server.register_prompt(prompt);
    ///
    /// let prompt = Prompt::new_with_args(
    ///    "personalized_greeting",
    ///    Some("A personalized greeting prompt"),
    ///    vec![PromptArgument {
    ///           name: "person".to_string(),
    ///           description: Some("The name of the person to greet".to_string()),
    ///           required: Some(true),
    ///    }],
    ///    |args| {
    ///         let args = args.ok_or(Error::MissingArgument("name".to_string()))?;
    ///         let name = args
    ///             .get("name")
    ///             .and_then(Value::as_str)
    ///             .ok_or(Error::MissingArgument("name".to_string()))?;
    ///         Ok(vec![PromptMessage::new_text(
    ///             PromptMessageRole::Assistant,
    ///             format!("Hello, {}!", name),
    ///         )])
    ///    }
    /// );
    /// server.register_prompt(prompt);
    ///
    /// ```
    pub fn register_prompt(&mut self, prompt: Prompt) {
        self.prompts.insert(prompt.name.clone(), prompt);
    }

    /// Register a resource with a callback to provide its contents.
    /// ```rust
    /// # use asimov_server::http::mcp::{Server, Resource};
    /// # use rmcp::model::ResourceContents;
    /// # let mut server = Server::new();
    /// let resource = Resource::new(
    ///     "file:///foo/bar/baz.txt",
    ///     "baz.txt",
    ///     Some("An example file"),
    ///     Some("text/plain"),
    ///     None,
    ///     || {
    ///         Ok(vec![ResourceContents::text(
    ///             "Hello, world!",
    ///             "file:///foo/bar/baz.txt",
    ///         )])
    ///     },
    /// );
    /// server.register_resource(resource);
    /// ```
    pub fn register_resource(&mut self, resource: Resource) {
        self.resources.insert(resource.uri.clone(), resource);
    }

    /// Register a resource template.
    pub fn register_resource_template(&mut self, template: ResourceTemplate) {
        self.resource_templates
            .insert(template.name.clone(), template);
    }

    /// Register a tool with a callback to handle tool calls.
    /// ```rust
    /// # use asimov_server::http::mcp::{Server, Tool};
    /// # use rmcp::model::Content;
    /// # let mut server = Server::new();
    /// let tool = Tool::new("frobnicate", Some("Does some processing"), || {
    ///     std::thread::sleep(std::time::Duration::from_millis(10));
    ///     Ok(vec![Content::text("Processing is done")])
    /// });
    /// server.register_tool(tool);
    /// ```
    pub fn register_tool(&mut self, tool: Tool) {
        self.tools.insert(tool.name.clone(), tool);
    }
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
            .values()
            .map(|prompt| model::Prompt {
                name: prompt.name.clone(),
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
            .values()
            .map(|resource| Annotated {
                raw: RawResource {
                    name: resource.name.clone(),
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
            .values()
            .map(|template| model::ResourceTemplate {
                raw: model::RawResourceTemplate {
                    name: template.name.clone(),
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
            .values()
            .map(|tool| model::Tool {
                name: Cow::from(tool.name.clone()),
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

#[cfg(test)]
mod test {
    use super::{Error, Prompt, Server};
    use crate::http::mcp::{Provider, Resource, Tool};
    use rmcp::model::{
        self, Content, PromptArgument, PromptMessage, PromptMessageRole, RawResource,
        ResourceContents,
    };
    use serde_json::{Map, Value};

    #[tokio::test]
    async fn test_prompts() {
        let mut server = Server::new();
        let prompt = Prompt::new("greeting", Some("A simple greeting prompt"), || {
            Ok(vec![PromptMessage::new_text(
                PromptMessageRole::Assistant,
                "Hello, world!",
            )])
        });
        server.register_prompt(prompt);

        let prompt = Prompt::new_with_args(
            "personalized_greeting",
            Some("A personalized greeting prompt"),
            vec![PromptArgument {
                name: "person".to_string(),
                description: Some("The name of the person to greet".to_string()),
                required: Some(true),
            }],
            |args| {
                let args = args.ok_or(Error::MissingArgument("name".to_string()))?;
                let name = args
                    .get("name")
                    .and_then(Value::as_str)
                    .ok_or(Error::MissingArgument("name".to_string()))?;
                Ok(vec![PromptMessage::new_text(
                    PromptMessageRole::Assistant,
                    format!("Hello, {}!", name),
                )])
            },
        );
        server.register_prompt(prompt);

        let (prompts, _) = server.list_prompts(None).await.unwrap();
        assert_eq!(
            prompts,
            vec![
                model::Prompt::new("greeting", Some("A simple greeting prompt"), None),
                model::Prompt::new(
                    "personalized_greeting",
                    Some("A personalized greeting prompt"),
                    Some(vec![PromptArgument {
                        name: "person".into(),
                        description: Some("The name of the person to greet".into()),
                        required: Some(true)
                    }])
                )
            ]
        );

        let mut args = Map::new();
        args.insert("name".to_string(), "Foobar".into());

        let (result, _) = server
            .get_prompt("personalized_greeting".to_string(), Some(args))
            .await
            .unwrap();
        assert_eq!(
            result,
            vec![PromptMessage::new_text(
                PromptMessageRole::Assistant,
                "Hello, Foobar!"
            )]
        );
    }

    #[tokio::test]
    async fn test_resources() {
        let mut server = Server::new();
        let resource = Resource::new(
            "file:///foo/bar/baz.txt",
            "baz.txt",
            Some("An example file"),
            Some("text/plain"),
            None,
            || {
                Ok(vec![ResourceContents::text(
                    "Hello, world!",
                    "file:///foo/bar/baz.txt",
                )])
            },
        );

        server.register_resource(resource);

        let (resources, _) = server.list_resources(None).await.unwrap();
        assert_eq!(
            resources,
            vec![model::Resource {
                raw: RawResource {
                    uri: "file:///foo/bar/baz.txt".into(),
                    name: "baz.txt".into(),
                    description: Some("An example file".into()),
                    mime_type: Some("text/plain".into()),
                    size: None,
                },
                annotations: None,
            }]
        );

        let result = server
            .read_resource("file:///foo/bar/baz.txt")
            .await
            .unwrap();
        assert_eq!(
            result,
            vec![ResourceContents::text(
                "Hello, world!",
                "file:///foo/bar/baz.txt"
            )]
        );
    }

    #[tokio::test]
    async fn test_tools() {
        let mut server = Server::new();
        let tool = Tool::new("frobnicate", Some("Does some processing"), || {
            std::thread::sleep(std::time::Duration::from_millis(10));
            Ok(vec![Content::text("Processing is done")])
        });
        server.register_tool(tool);

        let (tools, _) = server.list_tools(None).await.unwrap();
        assert_eq!(
            tools,
            vec![model::Tool::new(
                "frobnicate",
                "Does some processing",
                Map::new(),
            )],
        );

        let (result, _) = server.call_tool("frobnicate", None).await.unwrap();
        assert_eq!(result, vec![Content::text("Processing is done")]);
    }
}

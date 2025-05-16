// This is free and unencumbered software released into the public domain.

use rmcp::model::{
    Annotated, Content, Implementation, Prompt, PromptMessage, ProtocolVersion, RawResource,
    ResourceContents, ResourceTemplate, ServerCapabilities, Tool,
};
use serde_json::{Map, Value};

#[async_trait::async_trait]
pub trait Provider {
    type Error;

    fn protocol_version(&self) -> ProtocolVersion;
    fn capabilities(&self) -> ServerCapabilities;
    fn implementation(&self) -> Implementation;

    async fn list_prompts(
        &self,
        page: Option<String>,
    ) -> Result<(Vec<Prompt>, Option<String>), Self::Error>;
    async fn get_prompt(
        &self,
        name: String,
        arguments: Option<Map<String, Value>>,
    ) -> Result<(Vec<PromptMessage>, Option<String>), Self::Error>;

    async fn list_resources(
        &self,
        page: Option<String>,
    ) -> Result<(Vec<Annotated<RawResource>>, Option<String>), Self::Error>;
    async fn list_resource_templates(
        &self,
        page: Option<String>,
    ) -> Result<(Vec<ResourceTemplate>, Option<String>), Self::Error>;
    async fn read_resource(&self, uri: &str) -> Result<Vec<ResourceContents>, Self::Error>;

    async fn list_tools(
        &self,
        page: Option<String>,
    ) -> Result<(Vec<Tool>, Option<String>), Self::Error>;
    async fn call_tool(
        &self,
        name: &str,
        arguments: Option<Map<String, Value>>,
    ) -> Result<(Vec<Content>, Option<bool>), Self::Error>;
}

#[derive(Clone)]
pub struct StubProvider;

#[async_trait::async_trait]
impl Provider for StubProvider {
    type Error = ();

    fn protocol_version(&self) -> ProtocolVersion {
        ProtocolVersion::V_2025_03_26
    }

    fn capabilities(&self) -> ServerCapabilities {
        ServerCapabilities::builder().build()
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
        todo!()
    }

    async fn get_prompt(
        &self,
        _name: String,
        _arguments: Option<Map<String, Value>>,
    ) -> Result<(Vec<PromptMessage>, Option<String>), Self::Error> {
        todo!()
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

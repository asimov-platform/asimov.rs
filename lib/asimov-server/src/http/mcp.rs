// This is free and unencumbered software released into the public domain.

use std::sync::Arc;

use axum::{
    extract::State,
    http::request::Parts,
    response::{IntoResponse, Response},
    routing::post,
    Json, Router,
};
use rmcp::model::{
    Annotated, CallToolRequestParam, CallToolResult, ClientJsonRpcMessage, Content,
    GetPromptRequestParam, GetPromptResult, Implementation, JsonRpcResponse, JsonRpcVersion2_0,
    ListPromptsResult, ListResourceTemplatesResult, ListResourcesResult, ListToolsResult, Prompt,
    PromptMessage, ProtocolVersion, RawResource, ReadResourceResult, ResourceContents,
    ResourceTemplate, ServerCapabilities, ServerInfo, Tool,
};
use serde_json::{Map, Value};

/// See: https://modelcontextprotocol.io/specification/2025-03-26/basic/transports#streamable-http
pub fn routes<P: Clone>() -> Router<App<P>> {
    Router::new().route("/mcp", post(post_handler).get(get_handler))
}

#[derive(Clone)]
pub struct App<P> {
    pub provider: Arc<P + Send + Sync>,
}

// impl App {
//     pub fn new(provider: P) -> Self {
//         Self { provider }
//     }
// }

#[async_trait::async_trait]
pub trait Provider: Send + Sync {
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

async fn get_handler<P>(State(_app): State<App<P>>, _parts: Parts) -> Result<Response, Response> {
    Ok(Json(false).into_response())
}

async fn post_handler<P>(
    State(app): State<App<P>>,
    _parts: Parts,
    Json(message): Json<ClientJsonRpcMessage>,
) -> Result<Response, Response> {
    use rmcp::model::{ClientRequest::*, JsonRpcMessage::*};
    match message {
        Request(req) => match req.request {
            InitializeRequest(_req) => Ok(Json(JsonRpcResponse {
                jsonrpc: JsonRpcVersion2_0,
                id: req.id,
                result: ServerInfo {
                    protocol_version: app.provider.protocol_version(),
                    capabilities: app.provider.capabilities(),
                    server_info: app.provider.implementation(),
                    instructions: None,
                },
            })
            .into_response()),
            PingRequest(_req) => Ok(Json(JsonRpcResponse {
                jsonrpc: JsonRpcVersion2_0,
                id: req.id,
                result: {},
            })
            .into_response()),

            // Prompts
            ListPromptsRequest(list_req) => {
                let cursor = list_req.params.and_then(|opt| opt.cursor);
                let Ok((prompts, next_cursor)) = app.provider.list_prompts(cursor).await else {
                    todo!()
                };

                Ok(Json(JsonRpcResponse {
                    jsonrpc: JsonRpcVersion2_0,
                    id: req.id,
                    result: ListPromptsResult {
                        prompts,
                        next_cursor,
                    },
                })
                .into_response())
            }
            GetPromptRequest(get_req) => {
                let GetPromptRequestParam { name, arguments } = get_req.params;

                let Ok((messages, description)) = app.provider.get_prompt(name, arguments).await
                else {
                    todo!()
                };

                Ok(Json(JsonRpcResponse {
                    jsonrpc: JsonRpcVersion2_0,
                    id: req.id,
                    result: GetPromptResult {
                        messages,
                        description,
                    },
                })
                .into_response())
            }

            // Resources
            ListResourcesRequest(list_req) => {
                let cursor = list_req.params.and_then(|opt| opt.cursor);
                let Ok((resources, next_cursor)) = app.provider.list_resources(cursor).await else {
                    todo!()
                };

                Ok(Json(JsonRpcResponse {
                    jsonrpc: JsonRpcVersion2_0,
                    id: req.id,
                    result: ListResourcesResult {
                        resources,
                        next_cursor,
                    },
                })
                .into_response())
            }
            ListResourceTemplatesRequest(list_req) => {
                let cursor = list_req.params.and_then(|opt| opt.cursor);
                let Ok((resource_templates, next_cursor)) =
                    app.provider.list_resource_templates(cursor).await
                else {
                    todo!()
                };

                Ok(Json(JsonRpcResponse {
                    jsonrpc: JsonRpcVersion2_0,
                    id: req.id,
                    result: ListResourceTemplatesResult {
                        resource_templates,
                        next_cursor,
                    },
                })
                .into_response())
            }
            ReadResourceRequest(read_req) => {
                let uri = read_req.params.uri;

                let Ok(contents) = app.provider.read_resource(&uri).await else {
                    todo!()
                };

                Ok(Json(JsonRpcResponse {
                    jsonrpc: JsonRpcVersion2_0,
                    id: req.id,
                    result: ReadResourceResult { contents },
                })
                .into_response())
            }

            // Tools
            ListToolsRequest(list_req) => {
                let cursor = list_req.params.and_then(|opt| opt.cursor);
                let Ok((tools, next_cursor)) = app.provider.list_tools(cursor).await else {
                    todo!()
                };

                Ok(Json(JsonRpcResponse {
                    jsonrpc: JsonRpcVersion2_0,
                    id: req.id,
                    result: ListToolsResult { tools, next_cursor },
                })
                .into_response())
            }
            CallToolRequest(call_req) => {
                let CallToolRequestParam { name, arguments } = call_req.params;

                let Ok((content, is_error)) = app.provider.call_tool(&name, arguments).await else {
                    todo!()
                };

                Ok(Json(JsonRpcResponse {
                    jsonrpc: JsonRpcVersion2_0,
                    id: req.id,
                    result: CallToolResult { content, is_error },
                })
                .into_response())
            }

            CompleteRequest(_req) => todo!(),
            SetLevelRequest(_req) => todo!(),
            SubscribeRequest(_req) => todo!(),
            UnsubscribeRequest(_req) => todo!(),
        },
        Response(_resp) => todo!(),
        Notification(_not) => todo!(),
        Error(_err) => todo!(),
        BatchRequest(_items) => todo!(),
        BatchResponse(_items) => todo!(),
    }
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

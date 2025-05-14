// This is free and unencumbered software released into the public domain.

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
    PromptMessage, ProtocolVersion, RawResource, ReadResourceRequestParam, ReadResourceResult,
    ResourceContents, ResourceTemplate, ServerCapabilities, ServerInfo, Tool,
};
use serde_json::{json, Map, Value};

/// See: https://modelcontextprotocol.io/specification/2025-03-26/basic/transports#streamable-http
pub fn routes() -> Router {
    // TODO: return Router<impl Provider> or something like that so that the state can be provided
    // externally?
    let app = App {
        provider: StubProvider {},
    };
    Router::new().nest(
        "/mcp",
        Router::new().route("/", post(post_handler).get(get_handler).with_state(app)),
    )
}

pub trait Provider {
    type Error;

    fn protocol_version(&self) -> ProtocolVersion;
    fn capabilities(&self) -> ServerCapabilities;
    fn implementation(&self) -> Implementation;

    fn list_prompts(
        &self,
        page: Option<String>,
    ) -> impl Future<Output = Result<(Vec<Prompt>, Option<String>), Self::Error>>;
    fn get_prompt(
        &self,
        name: String,
        arguments: Option<Map<String, Value>>,
    ) -> impl Future<Output = Result<(Vec<PromptMessage>, Option<String>), Self::Error>>;

    fn list_resources(
        &self,
        page: Option<String>,
    ) -> impl Future<Output = Result<(Vec<Annotated<RawResource>>, Option<String>), Self::Error>>;
    fn list_resource_templates(
        &self,
        page: Option<String>,
    ) -> impl Future<Output = Result<(Vec<ResourceTemplate>, Option<String>), Self::Error>>;
    fn read_resource(
        &self,
        uri: &str,
    ) -> impl Future<Output = Result<(Vec<ResourceContents>), Self::Error>>;

    fn list_tools(
        &self,
        page: Option<String>,
    ) -> impl Future<Output = Result<(Vec<Tool>, Option<String>), Self::Error>>;
    fn call_tool(
        &self,
        name: &str,
        arguments: Option<Map<String, Value>>,
    ) -> impl Future<Output = Result<(Vec<Content>, Option<bool>), Self::Error>>;
}

#[derive(Clone)]
pub struct App<P: Provider> {
    provider: P,
}

impl<P: Provider> App<P> {
    pub fn new(provider: P) -> Self {
        Self { provider }
    }
}

async fn get_handler(
    State(app): State<App<impl Provider>>,
    parts: Parts,
) -> Result<Response, Response> {
    Ok(Json(false).into_response())
}

async fn post_handler(
    State(app): State<App<impl Provider>>,
    parts: Parts,
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

            CompleteRequest(req) => todo!(),
            SetLevelRequest(req) => todo!(),
            SubscribeRequest(req) => todo!(),
            UnsubscribeRequest(req) => todo!(),
        },
        Response(resp) => todo!(),
        Notification(not) => todo!(),
        Error(err) => todo!(),
        BatchRequest(items) => todo!(),
        BatchResponse(items) => todo!(),
    }
}

#[derive(Clone)]
pub struct StubProvider;

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
        page: Option<String>,
    ) -> Result<(Vec<Prompt>, Option<String>), Self::Error> {
        todo!()
    }

    async fn get_prompt(
        &self,
        name: String,
        arguments: Option<Map<String, Value>>,
    ) -> Result<(Vec<PromptMessage>, Option<String>), Self::Error> {
        todo!()
    }

    async fn list_resources(
        &self,
        page: Option<String>,
    ) -> Result<(Vec<Annotated<RawResource>>, Option<String>), Self::Error> {
        todo!()
    }

    async fn list_resource_templates(
        &self,
        page: Option<String>,
    ) -> Result<(Vec<ResourceTemplate>, Option<String>), Self::Error> {
        todo!()
    }

    async fn read_resource(&self, uri: &str) -> Result<(Vec<ResourceContents>), Self::Error> {
        todo!()
    }

    async fn list_tools(
        &self,
        page: Option<String>,
    ) -> Result<(Vec<Tool>, Option<String>), Self::Error> {
        todo!()
    }

    async fn call_tool(
        &self,
        name: &str,
        arguments: Option<Map<String, Value>>,
    ) -> Result<(Vec<Content>, Option<bool>), Self::Error> {
        todo!()
    }
}

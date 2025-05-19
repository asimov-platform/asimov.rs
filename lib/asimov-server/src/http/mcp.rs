// This is free and unencumbered software released into the public domain.

use axum::{
    extract::State,
    http::request::Parts,
    response::{IntoResponse, Response},
    routing::post,
    Json, Router,
};
use rmcp::model::{
    CallToolRequestParam, CallToolResult, ClientJsonRpcMessage, GetPromptRequestParam,
    GetPromptResult, JsonRpcResponse, JsonRpcVersion2_0, ListPromptsResult,
    ListResourceTemplatesResult, ListResourcesResult, ListToolsResult, ReadResourceResult,
    ServerInfo,
};

mod provider;
pub use provider::*;

mod server;
pub use server::*;

/// See: https://modelcontextprotocol.io/specification/2025-03-26/basic/transports#streamable-http
pub fn routes<P>() -> Router<P>
where
    P: Provider + Clone + Send + Sync + 'static,
{
    Router::new().route("/mcp", post(post_handler::<P>).get(get_handler::<P>))
}

async fn get_handler<P>(State(_provider): State<P>, _parts: Parts) -> Result<Response, Response>
where
    P: Provider,
{
    Ok(Json(false).into_response())
}

async fn post_handler<P>(
    State(provider): State<P>,
    _parts: Parts,
    Json(message): Json<ClientJsonRpcMessage>,
) -> Result<Response, Response>
where
    P: Provider,
{
    use rmcp::model::{ClientRequest::*, JsonRpcMessage::*};
    match message {
        Request(req) => match req.request {
            InitializeRequest(_req) => Ok(Json(JsonRpcResponse {
                jsonrpc: JsonRpcVersion2_0,
                id: req.id,
                result: ServerInfo {
                    protocol_version: provider.protocol_version(),
                    capabilities: provider.capabilities(),
                    server_info: provider.implementation(),
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
                let Ok((prompts, next_cursor)) = provider.list_prompts(cursor).await else {
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

                let Ok((messages, description)) = provider.get_prompt(name, arguments).await else {
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
                let Ok((resources, next_cursor)) = provider.list_resources(cursor).await else {
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
                    provider.list_resource_templates(cursor).await
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

                let Ok(contents) = provider.read_resource(&uri).await else {
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
                let Ok((tools, next_cursor)) = provider.list_tools(cursor).await else {
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

                let Ok((content, is_error)) = provider.call_tool(&name, arguments).await else {
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

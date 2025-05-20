// This is free and unencumbered software released into the public domain.

use asimov_server::http::mcp::Tool;
use axum::{http::StatusCode, Router};
use axum_test::TestServer;
use rmcp::model::{
    CallToolRequest, CallToolRequestMethod, CallToolRequestParam, CallToolResult, Content,
    Extensions, InitializeRequest, InitializeRequestParam, InitializeResultMethod,
    InitializedNotification, InitializedNotificationMethod, JsonRpcNotification, JsonRpcRequest,
    JsonRpcResponse, JsonRpcVersion2_0, ListToolsRequest, ListToolsRequestMethod, ListToolsResult,
    RequestId, ServerInfo,
};
use serde_json::json;
use tracing::debug;

#[tokio::test]
pub async fn test_mcp_lifecycle() {
    tracing_subscriber::fmt().init();

    let mut server = asimov_server::http::mcp::Server::default();

    server.register_tool(Tool::new_with_args(
        "text/transform",
        Some("Transform a piece of text"),
        json!({ "type": "object", "properties": { "text": { "type": "string" } } })
            .as_object()
            .unwrap()
            .clone(),
        |args| {
            let args = args.unwrap();
            let text = args.get("text").unwrap().as_str().unwrap();
            Ok(vec![Content::text(text.to_uppercase())])
        },
    ));

    let router = Router::new().merge(asimov_server::http::mcp::routes().with_state(server));

    let server = TestServer::new(router).unwrap();

    let req = JsonRpcRequest {
        jsonrpc: JsonRpcVersion2_0,
        id: RequestId::Number(1),
        request: InitializeRequest {
            method: InitializeResultMethod,
            extensions: Extensions::default(),
            params: InitializeRequestParam::default(),
        },
    };
    let req_str = serde_json::to_string(&req).unwrap();
    debug!("{req:?}: {req_str:?}");
    let resp = server.post("/mcp").json(&req).await;
    debug!("{resp:?}");
    let resp = resp.json::<JsonRpcResponse<ServerInfo>>();
    debug!("{resp:?}");

    let req = JsonRpcNotification {
        jsonrpc: JsonRpcVersion2_0,
        notification: InitializedNotification {
            method: InitializedNotificationMethod,
            extensions: Extensions::default(),
        },
    };
    let req_str = serde_json::to_string(&req).unwrap();
    debug!("{req:?}: {req_str:?}");
    let resp = server.post("/mcp").json(&req).await;
    debug!("{resp:?}");
    assert_eq!(resp.status_code(), StatusCode::ACCEPTED);
    assert_eq!(resp.as_bytes().len(), 0);

    let req = JsonRpcRequest {
        jsonrpc: JsonRpcVersion2_0,
        id: RequestId::Number(1),
        request: ListToolsRequest {
            method: ListToolsRequestMethod,
            extensions: Extensions::default(),
            params: None,
        },
    };
    let req_str = serde_json::to_string(&req).unwrap();
    debug!("{req:?}: {req_str:?}");
    let resp = server.post("/mcp").json(&req).await;
    debug!("{resp:?}");
    let resp = resp.json::<JsonRpcResponse<ListToolsResult>>();
    debug!("{resp:?}");

    let req = JsonRpcRequest {
        jsonrpc: JsonRpcVersion2_0,
        id: RequestId::Number(1),
        request: CallToolRequest {
            method: CallToolRequestMethod,
            extensions: Extensions::default(),
            params: CallToolRequestParam {
                name: "text/transform".into(),
                arguments: Some(json!({"text": "foobar"}).as_object().unwrap().clone()),
            },
        },
    };
    let req_str = serde_json::to_string(&req).unwrap();
    debug!("{req:?}: {req_str:?}");
    let resp = server.post("/mcp").json(&req).await;
    assert_eq!(resp.header("content-type"), "application/json");
    debug!("{resp:?}");
    let resp = resp.json::<JsonRpcResponse<CallToolResult>>();
    debug!("{resp:?}");
    assert_eq!(
        resp.result.content.first().unwrap().as_text().unwrap().text,
        "FOOBAR"
    )
}

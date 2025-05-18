// This is free and unencumbered software released into the public domain.

#![allow(unused_imports)]

mod nonstreaming;
mod streaming;

use axum::{
    Json, Router, extract,
    response::{IntoResponse, Response},
    routing::{delete, get, post},
};
use jiff::Timestamp;
use openai::schemas::{
    ChatCompletionDeleted, ChatCompletionList, ChatCompletionMessageList, CompletionUsage,
    CreateChatCompletionRequest_Variant2 as CreateChatCompletionRequest,
    CreateChatCompletionResponse, Metadata,
};

/// See: https://platform.openai.com/docs/api-reference/chat
pub fn routes() -> Router {
    Router::new()
        .route("/", get(list))
        .route("/{completion_id}", get(get_))
        .route("/{completion_id}/messages", get(get_messages))
        .route("/", post(create))
        .route("/{completion_id}", post(update))
        .route("/{completion_id}", delete(delete_))
}

/// See: https://platform.openai.com/docs/api-reference/chat/list
#[axum::debug_handler]
async fn list() -> Json<ChatCompletionList> {
    Json(ChatCompletionList {
        object: "list".into(),
        data: vec![], // TODO
        first_id: "".into(),
        last_id: "".into(),
        has_more: false,
    })
}

/// See: https://platform.openai.com/docs/api-reference/chat/get
#[axum::debug_handler]
async fn get_(extract::Path(_): extract::Path<String>) -> Json<CreateChatCompletionResponse> {
    Json(dummy_response()) // TODO
}

/// See: https://platform.openai.com/docs/api-reference/chat/getMessages
#[axum::debug_handler]
async fn get_messages(extract::Path(_): extract::Path<String>) -> Json<ChatCompletionMessageList> {
    Json(ChatCompletionMessageList {
        object: "list".into(),
        data: vec![], // TODO
        first_id: "".into(),
        last_id: "".into(),
        has_more: false,
    })
}

/// See: https://platform.openai.com/docs/api-reference/chat/create
#[axum::debug_handler]
async fn create(extract::Json(request): extract::Json<CreateChatCompletionRequest>) -> Response {
    if request.stream.unwrap_or_default() {
        streaming::create(request).await.into_response()
    } else {
        nonstreaming::create(request).await.into_response()
    }
}

/// See: https://platform.openai.com/docs/api-reference/chat/update
#[axum::debug_handler]
async fn update(
    extract::Path(_): extract::Path<String>,
    extract::Json(_): extract::Json<Metadata>,
) -> Json<CreateChatCompletionResponse> {
    Json(dummy_response()) // TODO
}

/// See: https://platform.openai.com/docs/api-reference/chat/delete
#[axum::debug_handler]
async fn delete_(extract::Path(_): extract::Path<String>) -> Json<ChatCompletionDeleted> {
    Json(ChatCompletionDeleted::default()) // TODO
}

fn dummy_response() -> CreateChatCompletionResponse {
    CreateChatCompletionResponse {
        id: super::util::generate_openai_id("chatcmpl"),
        object: "chat.completion".into(),
        created: Timestamp::now().as_second(),
        model: "gpt-4.1-2025-04-14".into(),
        choices: vec![],
        service_tier: None,
        system_fingerprint: None,
        usage: Some(CompletionUsage {
            prompt_tokens: 0,
            completion_tokens: 0,
            total_tokens: 0,
            prompt_tokens_details: Default::default(),
            completion_tokens_details: Default::default(),
        }),
    }
}

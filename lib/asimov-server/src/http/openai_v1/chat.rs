// This is free and unencumbered software released into the public domain.

#![allow(unused_imports)]

use axum::{
    Json, Router, extract,
    routing::{delete, get, post},
};
use openai::components::{
    ChatCompletionDeleted, ChatCompletionList, ChatCompletionMessageList,
    CreateChatCompletionRequest, CreateChatCompletionResponse, CreateCompletionRequest,
    CreateCompletionResponse, Metadata,
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
        object: "list".to_string(),
        data: vec![], // TODO
        first_id: String::from(""),
        last_id: String::from(""),
        has_more: false,
    })
}

/// See: https://platform.openai.com/docs/api-reference/chat/get
#[axum::debug_handler]
async fn get_(extract::Path(_): extract::Path<String>) -> Json<bool> {
    Json(false) // TODO
}

/// See: https://platform.openai.com/docs/api-reference/chat/getMessages
#[axum::debug_handler]
async fn get_messages(extract::Path(_): extract::Path<String>) -> Json<bool> {
    Json(false) // TODO
}

/// See: https://platform.openai.com/docs/api-reference/chat/create
#[axum::debug_handler]
async fn create(extract::Json(_): extract::Json<CreateChatCompletionRequest>) -> Json<bool> {
    Json(false) // TODO
}

/// See: https://platform.openai.com/docs/api-reference/chat/update
#[axum::debug_handler]
async fn update(
    extract::Path(_): extract::Path<String>,
    extract::Json(_): extract::Json<Metadata>,
) -> Json<bool> {
    Json(false) // TODO
}

/// See: https://platform.openai.com/docs/api-reference/chat/delete
#[axum::debug_handler]
async fn delete_(extract::Path(_): extract::Path<String>) -> Json<bool> {
    Json(false) // TODO
}

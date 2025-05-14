// This is free and unencumbered software released into the public domain.

#![allow(unused_imports)]

use axum::{
    Json, Router, extract,
    routing::{delete, get, post},
};
use jiff::Timestamp;
use openai::components::{
    ChatCompletionDeleted, ChatCompletionList, ChatCompletionMessageList, CompletionUsage,
    CreateChatCompletionRequest, CreateChatCompletionResponse, Metadata,
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
        first_id: "".to_string(),
        last_id: "".to_string(),
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
        object: "list".to_string(),
        data: vec![], // TODO
        first_id: "".to_string(),
        last_id: "".to_string(),
        has_more: false,
    })
}

/// See: https://platform.openai.com/docs/api-reference/chat/create
#[axum::debug_handler]
async fn create(
    extract::Json(_): extract::Json<CreateChatCompletionRequest>,
) -> Json<CreateChatCompletionResponse> {
    Json(dummy_response()) // TODO
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
        id: String::from("chatcmpl-B9MBs8CjcvOU2jLn4n570S5qMJKcT"),
        object: "chat.completion".to_string(),
        created: Timestamp::now().as_second(),
        model: String::from("gpt-4.1-2025-04-14"),
        choices: vec![],
        service_tier: None,
        system_fingerprint: String::from(""),
        usage: CompletionUsage {
            completion_tokens: 0,
            prompt_tokens: 0,
            total_tokens: 0,
            completion_tokens_details: Default::default(),
            prompt_tokens_details: Default::default(),
        },
    }
}

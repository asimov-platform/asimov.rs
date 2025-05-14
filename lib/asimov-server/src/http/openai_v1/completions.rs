// This is free and unencumbered software released into the public domain.

#![allow(unused_imports)]

use axum::{Json, Router, extract, routing::post};
use jiff::Timestamp;
use openai::components::{CompletionUsage, CreateCompletionRequest, CreateCompletionResponse};

/// See: https://platform.openai.com/docs/api-reference/completions
pub fn routes() -> Router {
    Router::new().route("/", post(create))
}

/// See: https://platform.openai.com/docs/api-reference/completions/create
#[axum::debug_handler]
async fn create(
    extract::Json(_): extract::Json<CreateCompletionRequest>,
) -> Json<CreateCompletionResponse> {
    // See: https://platform.openai.com/docs/api-reference/completions/object
    Json(CreateCompletionResponse {
        id: String::from("cmpl-uqkvlQyYK7bGYrRHQ0eXlWi7"), // TODO
        choices: vec![],
        created: Timestamp::now().as_second(),
        model: String::new(),
        system_fingerprint: String::new(),
        object: "text_completion".to_string(),
        usage: CompletionUsage {
            completion_tokens: 0,
            prompt_tokens: 0,
            total_tokens: 0,
            completion_tokens_details: Default::default(),
            prompt_tokens_details: Default::default(),
        },
    }) // TODO
}

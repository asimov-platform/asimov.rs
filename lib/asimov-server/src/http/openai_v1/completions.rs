// This is free and unencumbered software released into the public domain.

#![allow(unused_imports)]

use axum::{Json, Router, extract, routing::post};
use openai::components::{CreateCompletionRequest, CreateCompletionResponse};

/// See: https://platform.openai.com/docs/api-reference/completions
pub fn routes() -> Router {
    Router::new().route("/", post(create))
}

/// See: https://platform.openai.com/docs/api-reference/completions/create
#[axum::debug_handler]
async fn create(extract::Json(_): extract::Json<CreateCompletionRequest>) -> Json<bool> {
    Json(false) // TODO
}

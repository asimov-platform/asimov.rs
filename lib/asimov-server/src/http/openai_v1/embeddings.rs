// This is free and unencumbered software released into the public domain.

#![allow(unused_imports)]

use axum::{Json, Router, extract, routing::post};
use openai::components::{CreateEmbeddingRequest, CreateEmbeddingResponse};

/// See: https://platform.openai.com/docs/api-reference/embeddings
pub fn routes() -> Router {
    Router::new().route("/", post(create))
}

/// See: https://platform.openai.com/docs/api-reference/embeddings/create
#[axum::debug_handler]
async fn create(extract::Json(_): extract::Json<CreateEmbeddingRequest>) -> Json<bool> {
    Json(false) // TODO
}

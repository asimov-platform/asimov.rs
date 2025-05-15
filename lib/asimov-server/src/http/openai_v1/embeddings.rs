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
async fn create(
    extract::Json(_): extract::Json<CreateEmbeddingRequest>,
) -> Json<CreateEmbeddingResponse> {
    Json(CreateEmbeddingResponse {
        object: "list".into(),
        model: "text-embedding-ada-002".into(),
        data: vec![],
        usage: Default::default(),
    }) // TODO
}

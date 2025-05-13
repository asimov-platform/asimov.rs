// This is free and unencumbered software released into the public domain.

use axum::{Json, Router, routing::post};

/// See: https://platform.openai.com/docs/api-reference/embeddings
pub fn routes() -> Router {
    Router::new().route("/", post(create))
}

/// See: https://platform.openai.com/docs/api-reference/embeddings/create
#[axum::debug_handler]
async fn create() -> Json<bool> {
    Json(false) // TODO
}

// This is free and unencumbered software released into the public domain.

use axum::{Json, Router, routing::post};

/// See: https://platform.openai.com/docs/api-reference/images
pub fn routes() -> Router {
    Router::new()
        .route("/generations", post(create))
        .route("/edits", post(create_edit))
        .route("/variations", post(create_variation))
}

/// See: https://platform.openai.com/docs/api-reference/images/create
#[axum::debug_handler]
async fn create() -> Json<bool> {
    Json(false) // TODO
}

/// See: https://platform.openai.com/docs/api-reference/images/createEdit
#[axum::debug_handler]
async fn create_edit() -> Json<bool> {
    Json(false) // TODO
}

/// See: https://platform.openai.com/docs/api-reference/images/createVariation
#[axum::debug_handler]
async fn create_variation() -> Json<bool> {
    Json(false) // TODO
}

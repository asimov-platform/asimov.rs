// This is free and unencumbered software released into the public domain.

use axum::{Json, Router, extract, routing::get};
use openai::components::Model;

/// See: https://platform.openai.com/docs/api-reference/models
pub fn routes() -> Router {
    Router::new()
        .route("/", get(list))
        .route("/{model}", get(retrieve))
}

/// See: https://platform.openai.com/docs/api-reference/models/list
#[axum::debug_handler]
async fn list() -> Json<Vec<Model>> {
    Json(vec![]) // TODO
}

/// See: https://platform.openai.com/docs/api-reference/models/retrieve
#[axum::debug_handler]
async fn retrieve(extract::Path(model): extract::Path<String>) -> Json<Model> {
    // TODO
    Json(Model {
        id: model,
        object: "model".to_string(),
        created: 1686935002,
        owned_by: "openai".to_string(),
    })
}

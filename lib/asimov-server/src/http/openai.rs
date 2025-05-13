// This is free and unencumbered software released into the public domain.

use axum::{Json, Router, extract::Path, routing::get};
use openai::components::Model;

pub fn routes() -> Router {
    Router::new()
        .route("/v1/models", get(models_list))
        .route("/v1/models/{model}", get(models_retrieve))
}

/// See: https://platform.openai.com/docs/api-reference/models/list
#[axum::debug_handler]
async fn models_list() -> Json<Vec<Model>> {
    Json(vec![])
}

/// See: https://platform.openai.com/docs/api-reference/models/retrieve
#[axum::debug_handler]
async fn models_retrieve(Path(model): Path<String>) -> Json<Model> {
    Json(Model {
        id: model,
        object: "model".to_string(),
        created: 1686935002,
        owned_by: "openai".to_string(),
    })
}

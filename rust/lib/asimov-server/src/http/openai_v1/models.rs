// This is free and unencumbered software released into the public domain.

#![allow(unused_imports)]

use axum::{
    Json, Router,
    body::Body,
    extract,
    http::StatusCode,
    response::{IntoResponse, Response},
    routing::get,
};
use jiff::Timestamp;
use openai::schemas::{DeleteModelResponse, ListModelsResponse, Model};

/// See: https://platform.openai.com/docs/api-reference/models
pub fn routes() -> Router {
    Router::new()
        .route("/", get(list))
        .route("/{model}", get(retrieve))
}

/// See: https://platform.openai.com/docs/api-reference/models/list
#[axum::debug_handler]
async fn list() -> Json<ListModelsResponse> {
    Json(ListModelsResponse {
        object: "list".into(),
        data: vec![], // TODO
    })
}

/// See: https://platform.openai.com/docs/api-reference/models/retrieve
#[axum::debug_handler]
async fn retrieve(
    extract::Path(model): extract::Path<String>,
) -> Result<Json<Model>, RetrieveError> {
    if model.is_empty() {
        return Err(RetrieveError::NotFound);
    }

    Ok(Json(Model {
        id: model,
        object: "model".into(),
        created: Timestamp::now().as_second(), // TODO
        owned_by: "openai".into(),
    }))
}

#[derive(Debug, thiserror::Error)]
enum RetrieveError {
    #[error("no model specified")]
    NotFound,
}

impl IntoResponse for RetrieveError {
    fn into_response(self) -> Response {
        Response::builder()
            .status(StatusCode::NOT_FOUND)
            .body(Body::empty())
            .unwrap()
    }
}

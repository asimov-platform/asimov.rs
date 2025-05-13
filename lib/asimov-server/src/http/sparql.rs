// This is free and unencumbered software released into the public domain.

use axum::{
    Json, Router,
    routing::{get, post},
};

/// See: https://www.w3.org/TR/sparql12-protocol/
pub fn routes() -> Router {
    Router::new()
        .route("/sparql", get(query_via_get))
        .route("/sparql", post(query_via_post))
}

/// See: https://www.w3.org/TR/sparql12-protocol/#query-via-get
/// See: https://www.w3.org/TR/sparql12-service-description/
#[axum::debug_handler]
async fn query_via_get() -> Json<bool> {
    Json(false) // TODO
}

/// See: https://www.w3.org/TR/sparql12-protocol/#query-via-post-direct
#[axum::debug_handler]
async fn query_via_post() -> Json<bool> {
    Json(false) // TODO
}

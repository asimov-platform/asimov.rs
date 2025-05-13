// This is free and unencumbered software released into the public domain.

use axum::{Json, Router, routing::post};

/// See: https://graphql.org/learn/serving-over-http/
/// See: https://graphql.github.io/graphql-over-http/draft/
pub fn routes() -> Router {
    Router::new().route("/graphql", post(query_via_post))
}

/// See: https://graphql.github.io/graphql-over-http/draft/#sec-POST
#[axum::debug_handler]
async fn query_via_post() -> Json<bool> {
    Json(false) // TODO
}

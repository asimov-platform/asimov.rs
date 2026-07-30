// This is free and unencumbered software released into the public domain.

use axum::Json;

/// See: https://www.w3.org/TR/void/#well-known
#[axum::debug_handler]
pub async fn file() -> Json<bool> {
    Json(false) // TODO
}

// This is free and unencumbered software released into the public domain.

use axum::Router;

/// See: https://platform.openai.com/docs/api-reference/realtime
#[cfg(not(feature = "unstable"))]
pub fn routes() -> Router {
    Router::new()
}

/// See: https://platform.openai.com/docs/api-reference/realtime
#[cfg(feature = "unstable")]
pub fn routes() -> Router {
    Router::new() // TODO
}

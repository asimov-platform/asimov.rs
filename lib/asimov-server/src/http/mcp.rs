// This is free and unencumbered software released into the public domain.

use axum::{
    Router,
    routing::{get, post},
};

/// See: https://modelcontextprotocol.io/specification/draft/basic/transports#streamable-http
pub fn routes() -> Router {
    Router::new().route("/mcp", post(|| async {}))
}

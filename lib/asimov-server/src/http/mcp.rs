// This is free and unencumbered software released into the public domain.

use axum::{Router, routing::post};

/// See: https://modelcontextprotocol.io/specification/2025-03-26/basic/transports#streamable-http
pub fn routes() -> Router {
    Router::new().route("/mcp", post(|| async {}))
}

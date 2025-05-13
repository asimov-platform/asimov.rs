// This is free and unencumbered software released into the public domain.

use super::openai_v1;
use axum::Router;

/// See: https://platform.openai.com/docs/api-reference/introduction
pub fn routes() -> Router {
    Router::new().nest("/v1", openai_v1::routes())
}

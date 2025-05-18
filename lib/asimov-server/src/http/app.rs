// This is free and unencumbered software released into the public domain.

use axum::Router;
use axum_embed::{FallbackBehavior, ServeEmbed};

pub fn routes() -> Router {
    Router::new().nest_service(
        "/app",
        ServeEmbed::<Assets>::with_parameters(
            Some("404.html".to_string()),
            FallbackBehavior::NotFound,
            Some("index.html".to_string()),
        ),
    )
}

#[derive(Clone, rust_embed::RustEmbed)]
#[folder = "app/"]
struct Assets;

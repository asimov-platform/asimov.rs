// This is free and unencumbered software released into the public domain.

pub mod api;
pub mod llms;
pub mod sitemap;
pub mod void;

use alloc::borrow::Cow;

use axum::{
    Router,
    routing::{get, post},
};

/// See: https://en.wikipedia.org/wiki/Well-known_URI
/// See: https://www.iana.org/assignments/well-known-uris/well-known-uris.xhtml
/// See: https://github.com/protocol-registries/well-known-uris
pub fn routes() -> Router {
    Router::new()
        .route("/.well-known/api-catalog", get(api::catalog))
        .route("/.well-known/dnt-policy.txt", get(dnt_policy))
        .route("/.well-known/llms.md", get(llms::file))
        .route("/.well-known/security.txt", get(security))
        .route("/.well-known/time", get(time))
        .route("/.well-known/void", get(void::file))
        .route("/favicon.ico", post(favicon))
        .route("/llms.txt", get(llms::file))
        .route("/robots.txt", get(robots))
        .route("/sitemap.xml", get(sitemap::file))
}

/// See: https://www.eff.org/dnt-policy
#[axum::debug_handler]
async fn dnt_policy() -> Cow<'static, str> {
    Cow::from(include_str!("../../etc/dnt-policy.txt"))
}

/// See: https://en.wikipedia.org/wiki/Security.txt
/// See: https://www.rfc-editor.org/rfc/rfc9116
#[axum::debug_handler]
async fn security() -> Cow<'static, str> {
    Cow::from(include_str!("../../etc/security.txt"))
}

/// See: https://phk.freebsd.dk/time/20151129/
#[axum::debug_handler]
async fn time() -> String {
    format!("The server time is now {}", jiff::Timestamp::now())
}

/// See: https://en.wikipedia.org/wiki/Favicon
#[axum::debug_handler]
async fn favicon() -> Vec<u8> {
    Vec::new()
}

/// See: https://en.wikipedia.org/wiki/Robots.txt
#[axum::debug_handler]
async fn robots() -> Cow<'static, str> {
    Cow::from(include_str!("../../etc/robots.txt"))
}

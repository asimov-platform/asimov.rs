// This is free and unencumbered software released into the public domain.

use axum::{response::Json, routing::get, Router};
use tokio::net::{TcpListener, ToSocketAddrs};
use tokio_util::sync::CancellationToken;
use tower_http::cors::CorsLayer;
use tracing::info;

pub async fn start(addr: impl ToSocketAddrs, cancel: CancellationToken) -> std::io::Result<()> {
    let app = Router::new()
        .route("/", get(http_handler))
        .layer(CorsLayer::permissive());

    let listener = TcpListener::bind(addr).await?;

    info!(
        "Listening for HTTP requests on {}...",
        listener.local_addr().unwrap()
    );

    axum::serve(listener, app)
        .with_graceful_shutdown(cancel.cancelled_owned())
        .await
}

async fn http_handler() -> Json<&'static str> {
    Json("Hello, world!") // TODO
}

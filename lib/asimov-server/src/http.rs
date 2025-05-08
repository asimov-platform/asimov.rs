// This is free and unencumbered software released into the public domain.

use axum::{response::Json, routing::get, Router};
use std::io::Error;
use tokio::{
    net::{TcpListener, ToSocketAddrs},
    task::JoinHandle,
};
use tokio_util::sync::CancellationToken;
use tower_http::cors::CorsLayer;
use tracing::info;

pub async fn start(
    addr: impl ToSocketAddrs,
) -> std::io::Result<(JoinHandle<Result<(), Error>>, CancellationToken)> {
    let app = Router::new()
        .route("/", get(http_handler))
        .layer(CorsLayer::permissive());

    let listener = TcpListener::bind(addr).await?;

    info!(
        "Listening for HTTP requests on {}...",
        listener.local_addr().unwrap()
    );

    let token = CancellationToken::new();
    let token_copy = token.clone();

    let handle = tokio::spawn(async move {
        axum::serve(listener, app)
            .with_graceful_shutdown(token_copy.cancelled_owned())
            .await
    });

    Ok((handle, token))
}

async fn http_handler() -> Json<&'static str> {
    Json("Hello, world!") // TODO
}

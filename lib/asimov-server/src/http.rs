// This is free and unencumbered software released into the public domain.

mod graphql;
mod gsp;
mod openai;
mod openai_v1;
mod prometheus;
mod sparql;

use axum::{Router, response::Json, routing::get};
use tokio::net::{TcpListener, ToSocketAddrs};
use tokio_util::sync::CancellationToken;
use tower_http::cors::CorsLayer;
use tracing::info;

pub fn routes() -> Router {
    Router::new()
        .merge(graphql::routes())
        .merge(gsp::routes())
        .merge(openai::routes())
        .merge(prometheus::routes())
        .merge(sparql::routes())
        .layer(CorsLayer::permissive())
        .route("/", get(http_handler))
}

pub async fn start(addr: impl ToSocketAddrs, cancel: CancellationToken) -> std::io::Result<()> {
    let listener = TcpListener::bind(addr).await?;

    info!(
        "Listening for HTTP requests on {}...",
        listener.local_addr().unwrap()
    );

    axum::serve(listener, routes())
        .with_graceful_shutdown(cancel.cancelled_owned())
        .await
}

async fn http_handler() -> Json<&'static str> {
    Json("Hello, world!") // TODO
}

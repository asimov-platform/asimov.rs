// This is free and unencumbered software released into the public domain.

mod graphql;
mod gsp;
pub mod mcp;
mod openai;
mod openai_v1;
mod prometheus;
mod sparql;
mod well_known;

use axum::{response::Json, routing::get, Router};
use tokio::net::{TcpListener, ToSocketAddrs};
use tokio_util::sync::CancellationToken;
use tower_http::cors::CorsLayer;
use tracing::info;

pub fn routes() -> Router {
    let mcp_app = mcp::StubProvider {};
    Router::new()
        .merge(graphql::routes())
        .merge(gsp::routes())
        .merge(mcp::routes().with_state(mcp_app))
        .merge(openai::routes())
        .merge(prometheus::routes())
        .merge(sparql::routes())
        .merge(well_known::routes())
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

// This is free and unencumbered software released into the public domain.

mod graphql;
mod gsp;
mod mcp;
mod openai;
mod openai_v1;
mod prometheus;
mod sparql;
mod well_known;

use axum::{Router, body::Body, response::Json, routing::get};
use http::Request;
use tokio::net::{TcpListener, ToSocketAddrs};
use tokio_util::sync::CancellationToken;
use tower_http::{
    cors::CorsLayer,
    trace::{DefaultMakeSpan, TraceLayer},
};
use tracing::{Span, info};

pub fn routes() -> Router {
    Router::new()
        .merge(graphql::routes())
        .merge(gsp::routes())
        .merge(mcp::routes())
        .merge(openai::routes())
        .merge(prometheus::routes())
        .merge(sparql::routes())
        .merge(well_known::routes())
        .layer(CorsLayer::permissive())
        .layer(
            TraceLayer::new_for_http()
                .make_span_with(DefaultMakeSpan::new().include_headers(true))
                .on_request(|request: &Request<Body>, _span: &Span| {
                    tracing::info!(
                        "Received a {} {} request",
                        request.method(),
                        request.uri().path()
                    );
                }),
        )
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

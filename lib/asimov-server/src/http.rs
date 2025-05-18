// This is free and unencumbered software released into the public domain.

mod graphql;
mod gsp;
mod mcp;
mod openai;
mod openai_v1;
mod prometheus;
mod sparql;
mod well_known;

use axum::{Router, response::Json, routing::get};
use tokio::net::{TcpListener, ToSocketAddrs};
use tokio_util::sync::CancellationToken;
use tower_http::cors::CorsLayer;

pub fn routes() -> Router {
    let router = Router::new()
        .merge(graphql::routes())
        .merge(gsp::routes())
        .merge(mcp::routes())
        .merge(openai::routes())
        .merge(prometheus::routes())
        .merge(sparql::routes())
        .merge(well_known::routes())
        .layer(CorsLayer::permissive());

    #[cfg(feature = "tracing")]
    let router = router.layer(
        tower_http::trace::TraceLayer::new_for_http()
            .make_span_with(tower_http::trace::DefaultMakeSpan::new().include_headers(true))
            .on_request(
                |request: &http::Request<axum::body::Body>, _span: &tracing::Span| {
                    tracing::info!(
                        "Received a {} {} request",
                        request.method(),
                        request.uri().path()
                    );
                },
            ),
    );

    router.route("/", get(http_handler))
}

pub async fn start(addr: impl ToSocketAddrs, cancel: CancellationToken) -> std::io::Result<()> {
    let listener = TcpListener::bind(addr).await?;

    #[cfg(feature = "tracing")]
    tracing::info!(
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

// This is free and unencumbered software released into the public domain.

use axum::{response::Json, routing::get, Router};
use axum_prometheus::PrometheusMetricLayer;
use std::time::Duration;
use tokio::net::{TcpListener, ToSocketAddrs};
use tokio_util::sync::CancellationToken;
use tower_http::cors::CorsLayer;
use tracing::info;

pub async fn start(addr: impl ToSocketAddrs, cancel: CancellationToken) -> std::io::Result<()> {
    let (prometheus_layer, metric_handle) = PrometheusMetricLayer::pair();

    let app = Router::new()
        .route("/fast", get(|| async {}))
        .route(
            "/slow",
            get(|| async {
                tokio::time::sleep(Duration::from_secs(1)).await;
            }),
        )
        .route("/metrics", get(|| async move { metric_handle.render() }))
        .route("/", get(http_handler))
        .layer(prometheus_layer)
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

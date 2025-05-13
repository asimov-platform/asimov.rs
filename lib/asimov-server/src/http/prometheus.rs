// This is free and unencumbered software released into the public domain.

use axum::{Router, routing::get};
use axum_prometheus::PrometheusMetricLayer;
use std::time::Duration;

/// See: https://prometheus.io/docs/instrumenting/exposition_formats/
pub fn routes() -> Router {
    let (prometheus_layer, metric_handle) = PrometheusMetricLayer::pair();
    Router::new()
        .route("/metrics", get(|| async move { metric_handle.render() }))
        .route("/fast", get(|| async {}))
        .route(
            "/slow",
            get(|| async {
                tokio::time::sleep(Duration::from_secs(1)).await;
            }),
        )
        .layer(prometheus_layer)
}

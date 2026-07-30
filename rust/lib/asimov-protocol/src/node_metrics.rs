// This is free and unencumbered software released into the public domain.

use iroh_metrics::{Counter, MetricsGroup};

/// Metrics for the node protocol.
#[derive(Debug, Default, MetricsGroup)]
#[metrics(name = "ping")]
pub struct NodeMetrics {
    /// The count of valid ping messages sent.
    pub pings_sent: Counter,

    /// The count of valid ping messages received.
    pub pings_recv: Counter,
}

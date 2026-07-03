// This is free and unencumbered software released into the public domain.

//! The internode ping protocol.

use alloc::{boxed::Box, sync::Arc};
use core::{error::Error, result::Result};
use iroh::{
    Endpoint, EndpointAddr,
    endpoint::Connection,
    protocol::{AcceptError, ProtocolHandler},
};
use iroh_metrics::{Counter, MetricsGroup};
use tokio::time::{Duration, Instant};

/// The ALPN string for the ping protocol.
pub const PING_ALPN: &[u8] = b"asimov/ping";

/// Metrics for the ping protocol.
#[derive(Debug, Default, MetricsGroup)]
#[metrics(name = "ping")]
pub struct PingMetrics {
    /// The count of valid ping messages sent.
    pub pings_sent: Counter,

    /// The count of valid ping messages received.
    pub pings_recv: Counter,
}

/// The ping protocol for use with `iroh::protocol::Router`.
#[derive(Debug, Clone)]
pub struct PingProtocol {
    /// Shared state for use across incoming connections.
    metrics: Arc<PingMetrics>,
}

impl Default for PingProtocol {
    fn default() -> Self {
        Self::new()
    }
}

impl PingProtocol {
    /// Creates a new ping protocol state.
    pub fn new() -> Self {
        Self {
            metrics: Arc::new(PingMetrics::default()),
        }
    }

    /// Returns a handle to the ping metrics.
    pub fn metrics(&self) -> &Arc<PingMetrics> {
        &self.metrics
    }

    /// Sends a ping on the provided endpoint to a given node address.
    pub async fn ping(
        &self,
        endpoint: &Endpoint,
        addr: EndpointAddr,
    ) -> Result<Duration, Box<dyn Error>> {
        // Open a connection to the accepting node:
        let conn = endpoint.connect(addr, PING_ALPN).await?;

        // Open a bidirectional QUIC stream:
        let (mut send, mut recv) = conn.open_bi().await?;

        // Begin measuring elapsed time:
        let start = Instant::now();

        // Send the PING request and finish the send stream:
        send.write_all(b"PING").await?;
        send.finish()?;

        // Read the PONG response:
        let response = recv.read_to_end(4).await?;
        assert_eq!(&response, b"PONG");

        // Measure the duration of this interaction:
        let duration = start.elapsed();

        // Update the metrics counters:
        self.metrics.pings_sent.inc();

        // Close the connection explicitly and gracefully:
        conn.close(0u32.into(), b"BYE!");

        Ok(duration)
    }
}

impl ProtocolHandler for PingProtocol {
    /// Each incoming connection for our ALPN results in a call to `accept`.
    ///
    /// The returned future runs on a newly spawned Tokio task, so it can run
    /// indefinitely as long as the connection remains open.
    async fn accept(&self, connection: Connection) -> n0_error::Result<(), AcceptError> {
        let node_id = connection.remote_id();
        #[cfg(feature = "std")]
        std::eprintln!("Accepted a connection from node {node_id}"); // DEBUG

        // Expect the connecting peer to open a bidirectional QUIC stream:
        let (mut send, mut recv) = connection.accept_bi().await?;

        // Read the PING request:
        let req = recv.read_to_end(4).await.map_err(AcceptError::from_err)?;
        assert_eq!(&req, b"PING");

        // Update the metrics counters:
        self.metrics.pings_recv.inc();

        // Send the PONG response and finish the send stream:
        send.write_all(b"PONG")
            .await
            .map_err(AcceptError::from_err)?;
        send.finish()?;

        // Wait for the remote end to explicitly and gracefully close the
        // connection after receiving our response:
        connection.closed().await;

        Ok(())
    }
}

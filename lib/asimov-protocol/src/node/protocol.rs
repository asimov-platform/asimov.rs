// This is free and unencumbered software released into the public domain.

//! The internode protocol.

use super::NodeMetrics;
use alloc::{boxed::Box, sync::Arc};
use core::{error::Error, result::Result};
use iroh::{
    Endpoint, EndpointAddr,
    endpoint::Connection,
    protocol::{AcceptError, ProtocolHandler},
};
use tokio::time::{Duration, Instant};

/// The ALPN string for the node protocol.
pub const NODE_ALPN: &[u8] = b"asimov/node";

/// The node protocol for use with `Router`.
#[derive(Debug, Clone)]
pub struct NodeProtocol {
    /// Shared state for use across incoming connections.
    metrics: Arc<NodeMetrics>,
}

impl Default for NodeProtocol {
    fn default() -> Self {
        Self::new()
    }
}

impl NodeProtocol {
    /// Creates a new node protocol state.
    pub fn new() -> Self {
        Self {
            metrics: Arc::new(NodeMetrics::default()),
        }
    }

    /// Returns a handle to the node metrics.
    pub fn metrics(&self) -> &Arc<NodeMetrics> {
        &self.metrics
    }

    /// Sends a ping on the provided endpoint to a given node address.
    pub async fn ping(
        &self,
        self_endpoint: &Endpoint,
        peer_addr: EndpointAddr,
    ) -> Result<Duration, Box<dyn Error>> {
        // Open a connection to the accepting node:
        let conn = self_endpoint.connect(peer_addr, NODE_ALPN).await?;

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

impl ProtocolHandler for NodeProtocol {
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

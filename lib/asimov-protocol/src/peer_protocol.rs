// This is free and unencumbered software released into the public domain.

//! The peer-to-peer protocol.

use crate::{Message, MessageRecv, MessageSend, NodeMetrics, PeerAccept};
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
    pub async fn _ping(
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

        // Send the ping request and finish the send stream:
        let request = postcard::to_stdvec(&Message::Ping)?;
        send.write_all(request.as_slice()).await?;
        send.finish()?;

        // Read the ping response:
        let response = recv.read_to_end(1024).await?;
        let response: Message = postcard::from_bytes(&response)?;
        assert_eq!(response, Message::Ping);

        // Measure the duration of this interaction:
        let duration = start.elapsed();

        // Update the metrics counters:
        self.metrics.pings_sent.inc();

        // Close the connection explicitly and gracefully:
        conn.close(0u32.into(), b"");

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
        let state: PeerAccept = connection.into();
        let state = state.recv_hello().await.map_err(AcceptError::from_err)?;
        let state = state.send_hello().await.map_err(AcceptError::from_err)?;

        let mut connection = state.into_connection();

        let mut is_alive = true;
        while is_alive {
            let request = connection.recv().await.map_err(AcceptError::from_err)?;
            let response: Message = match request {
                Message::Hello(hello) => {
                    Message::Hello(hello) // TODO
                },

                Message::Bye => {
                    is_alive = false;
                    Message::Bye
                },

                Message::Ping => {
                    // Update the metrics counters:
                    self.metrics.pings_recv.inc();

                    Message::Ping
                },

                _ => unimplemented!(), // TODO
            };
            connection
                .send(response)
                .await
                .map_err(AcceptError::from_err)?;
        }

        // Send the response and finish the send stream:
        connection.send.finish()?;

        // Wait for the remote end to explicitly and gracefully close the
        // connection after receiving our response:
        connection.inner.closed().await;

        Ok(())
    }
}

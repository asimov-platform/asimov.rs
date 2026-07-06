// This is free and unencumbered software released into the public domain.

//! The peer-to-peer protocol.

use crate::{Message, MessageRecv, MessageSend, NodeMetrics, PeerAccept};
use alloc::sync::Arc;
use iroh::{
    endpoint::Connection,
    protocol::{AcceptError, ProtocolHandler},
};

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

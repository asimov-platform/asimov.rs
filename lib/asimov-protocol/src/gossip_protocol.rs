// This is free and unencumbered software released into the public domain.

//! The internode gossip protocol.

use iroh::Endpoint;
use iroh_gossip::{ALPN, Gossip};

/// The ALPN string for the gossip protocol.
pub const GOSSIP_ALPN: &[u8] = ALPN;

/// The gossip protocol for use with `Router`.
#[derive(Debug, Clone)]
pub struct GossipProtocol(pub Gossip);

impl GossipProtocol {
    pub fn new(endpoint: Endpoint) -> Self {
        Self(Gossip::builder().spawn(endpoint))
    }
}

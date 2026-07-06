// This is free and unencumbered software released into the public domain.

use crate::{
    BindError, DefaultPreset, GOSSIP_ALPN, GossipProtocol, PingError, StartError, SubscribeError,
    TerminateError, Topic, TopicSubscription,
};
use alloc::vec::Vec;
use core::{result::Result, time::Duration};
use iroh::{Endpoint, EndpointAddr, EndpointId, endpoint::EndpointClosed, protocol::Router};

#[derive(Debug)]
pub struct Node<State = state::Building>(State);

impl Default for Node {
    fn default() -> Self {
        Self(state::Building {
            endpoint: Endpoint::builder(DefaultPreset),
        })
    }
}

impl Node<state::Building> {
    pub async fn bind(self) -> Result<Node<state::Bound>, BindError> {
        Ok(Node(state::Bound {
            endpoint: self.0.endpoint.bind().await?,
        }))
    }
}

impl Node<state::Bound> {
    pub fn endpoint_addr(&self) -> EndpointAddr {
        self.endpoint().addr()
    }

    pub fn endpoint(&self) -> &Endpoint {
        &self.0.endpoint
    }

    pub async fn start(self) -> Result<Node<state::Running>, StartError> {
        let endpoint = self.0.endpoint;
        endpoint.online().await;
        let node = NodeProtocol::new();
        let gossip = GossipProtocol::new(endpoint.clone());
        let router = Router::builder(endpoint)
            .accept(NODE_ALPN, node.clone())
            .accept(GOSSIP_ALPN, gossip.0.clone())
            .spawn();
        Ok(Node(state::Running {
            router,
            node,
            gossip,
            peers: Vec::new(),
        }))
    }
}

impl Node<state::Running> {
    pub fn endpoint_addr(&self) -> EndpointAddr {
        self.endpoint().addr()
    }

    pub fn endpoint(&self) -> &Endpoint {
        self.0.router.endpoint()
    }

    pub fn is_closed(&self) -> bool {
        self.endpoint().is_closed()
    }

    pub fn closed(&self) -> EndpointClosed {
        self.endpoint().closed()
    }

    pub async fn terminate(self) -> Result<Node<state::Terminating>, TerminateError> {
        let router = self.0.router;
        router.shutdown().await?;
        Ok(Node(state::Terminating { router }))
    }

    pub async fn online(&self) {
        self.endpoint().online().await
    }

    pub fn add_peer(&mut self, endpoint: impl Into<EndpointId>) {
        self.0.peers.push(endpoint.into());
    }

    pub async fn ping(&self, peer_addr: impl Into<EndpointAddr>) -> Result<Duration, PingError> {
        Ok(self
            .0
            .node
            .ping(self.0.router.endpoint(), peer_addr.into())
            .await?)
    }

    pub async fn subscribe(
        &self,
        topic: impl Into<Topic>,
    ) -> Result<TopicSubscription, SubscribeError> {
        let topic_id = topic.into().id();
        Ok(self
            .0
            .gossip
            .0
            .subscribe(topic_id, self.0.peers.clone())
            .await?
            .into())
    }

    pub async fn subscribe_and_join(
        &self,
        topic: impl Into<Topic>,
    ) -> Result<TopicSubscription, SubscribeError> {
        let topic_id = topic.into().id();
        Ok(self
            .0
            .gossip
            .0
            .subscribe_and_join(topic_id, self.0.peers.clone())
            .await?
            .into())
    }
}

impl Node<state::Terminating> {}

mod hello;
pub use hello::*;

mod feature_set;
pub use feature_set::*;

mod metrics;
pub use metrics::*;

mod protocol;
pub use protocol::*;

mod request;
pub use request::*;

mod response;
pub use response::*;

pub mod state;

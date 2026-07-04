// This is free and unencumbered software released into the public domain.

use crate::{
    BindError, DefaultPreset, GOSSIP_ALPN, GossipProtocol, PING_ALPN, PingError, PingProtocol,
    StartError, SubscribeError, Topic, TopicSubscription,
};
use alloc::vec::Vec;
use core::{result::Result, time::Duration};
use iroh::{Endpoint, EndpointAddr, EndpointId, endpoint::EndpointClosed, protocol::Router};

pub mod state {
    use crate::{Endpoint, GossipProtocol, PingProtocol, Router};
    use alloc::vec::Vec;
    use iroh::{EndpointId, endpoint::Builder as EndpointBuilder};

    pub struct Building {
        pub(crate) endpoint: EndpointBuilder,
    }

    pub struct Bound {
        pub(crate) endpoint: Endpoint,
    }

    pub struct Running {
        pub(crate) router: Router,
        pub(crate) pinger: PingProtocol,
        pub(crate) gossiper: GossipProtocol,
        pub(crate) peers: Vec<EndpointId>,
    }

    pub struct Terminating;
}

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
        let ping = PingProtocol::new();
        let gossip = GossipProtocol::new(endpoint.clone());
        let router = Router::builder(endpoint)
            .accept(PING_ALPN, ping.clone())
            .accept(GOSSIP_ALPN, gossip.0.clone())
            .spawn();
        Ok(Node(state::Running {
            router,
            pinger: ping,
            gossiper: gossip,
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

    pub async fn close(&self) {
        self.endpoint().close().await
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
            .pinger
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
            .gossiper
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
            .gossiper
            .0
            .subscribe_and_join(topic_id, self.0.peers.clone())
            .await?
            .into())
    }
}

impl Node<state::Terminating> {}

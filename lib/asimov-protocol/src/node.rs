// This is free and unencumbered software released into the public domain.

use crate::{
    BindError, ConnectError, DefaultPreset, GOSSIP_ALPN, GossipProtocol, NODE_ALPN, NodeProtocol,
    PeerConnect, PeerConnection, PingError, StartError, SubscribeError, TerminateError, Topic,
    TopicSubscription, node_state::*,
};
use alloc::vec::Vec;
use core::{result::Result, time::Duration};
use iroh::{Endpoint, EndpointAddr, EndpointId, endpoint::EndpointClosed, protocol::Router};

#[derive(Debug)]
pub struct Node<State = Building>(State);

impl Default for Node {
    fn default() -> Self {
        Self(Building {
            endpoint: Endpoint::builder(DefaultPreset),
        })
    }
}

impl Node<Building> {
    pub async fn bind(self) -> Result<Node<Bound>, BindError> {
        Ok(Node(Bound {
            endpoint: self.0.endpoint.bind().await?,
        }))
    }
}

impl Node<Bound> {
    pub fn endpoint_addr(&self) -> EndpointAddr {
        self.endpoint().addr()
    }

    pub fn endpoint(&self) -> &Endpoint {
        &self.0.endpoint
    }

    pub async fn start(self) -> Result<Node<Running>, StartError> {
        let endpoint = self.0.endpoint;
        endpoint.online().await;
        let node = NodeProtocol::new();
        let gossip = GossipProtocol::new(endpoint.clone());
        let router = Router::builder(endpoint)
            .accept(NODE_ALPN, node.clone())
            .accept(GOSSIP_ALPN, gossip.0.clone())
            .spawn();
        Ok(Node(Running {
            router,
            node,
            gossip,
            peers: Vec::new(),
        }))
    }
}

impl Node<Running> {
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

    pub async fn terminate(self) -> Result<Node<Terminating>, TerminateError> {
        let router = self.0.router;
        router.shutdown().await?;
        Ok(Node(Terminating { router }))
    }

    pub async fn online(&self) {
        self.endpoint().online().await
    }

    pub fn add_peer(&mut self, endpoint: impl Into<EndpointId>) {
        self.0.peers.push(endpoint.into());
    }

    pub async fn ping(&self, peer_addr: impl Into<EndpointAddr>) -> Result<Duration, PingError> {
        let mut connection = self.connect(peer_addr).await?;
        let rtt = connection.ping().await?;
        Ok(rtt)
    }

    pub async fn connect(
        &self,
        peer_addr: impl Into<EndpointAddr>,
    ) -> Result<PeerConnection, ConnectError> {
        let endpoint = self.0.router.endpoint();

        let state: PeerConnect = endpoint.connect(peer_addr, NODE_ALPN).await?.into();
        let state = state.send_hello().await?;
        let state = state.recv_hello().await?;

        Ok(state.into_connection())
    }

    pub async fn subscribe(
        &self,
        topic: impl Into<Topic>,
    ) -> Result<TopicSubscription, SubscribeError> {
        let topic_id = topic.into().id();
        let gossip = &self.0.gossip;
        Ok(gossip
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
        let gossip = &self.0.gossip;
        Ok(gossip
            .0
            .subscribe_and_join(topic_id, self.0.peers.clone())
            .await?
            .into())
    }
}

impl Node<Terminating> {}

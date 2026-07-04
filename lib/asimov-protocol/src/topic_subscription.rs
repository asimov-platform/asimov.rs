// This is free and unencumbered software released into the public domain.

use crate::PublishError;
use bytes::Bytes;
pub use iroh_gossip::api::{GossipReceiver, GossipSender, GossipTopic};

#[derive(Debug)]
pub struct TopicSubscription(pub GossipTopic);

impl TopicSubscription {
    pub fn split(self) -> (GossipSender, GossipReceiver) {
        self.0.split()
    }

    pub async fn publish(&mut self, payload: impl Into<Bytes>) -> Result<(), PublishError> {
        Ok(self.0.broadcast(payload.into()).await?)
    }
}

impl From<GossipTopic> for TopicSubscription {
    fn from(input: GossipTopic) -> Self {
        Self(input)
    }
}

impl From<TopicSubscription> for GossipTopic {
    fn from(input: TopicSubscription) -> Self {
        input.0
    }
}

// This is free and unencumbered software released into the public domain.

#![no_std]
#![forbid(unsafe_code)]
//#![allow(unused)]
#![cfg_attr(docsrs, feature(doc_cfg))]

extern crate alloc;

#[cfg(feature = "std")]
extern crate std;

#[doc(hidden)]
pub use iroh;

pub use iroh::{
    Endpoint, EndpointAddr, EndpointId, PublicKey, SecretKey,
    endpoint::presets::Preset,
    protocol::{Router, RouterBuilder},
};
pub use iroh_gossip::{
    TopicId,
    api::{GossipReceiver, GossipSender, GossipTopic},
};
pub use iroh_tickets::{ParseError as TicketParsingError, Ticket, endpoint::EndpointTicket};

mod errors;
pub use errors::*;

mod gossip_protocol;
pub use gossip_protocol::*;

mod message;
pub use message::*;

mod message_header;
pub use message_header::*;

mod message_recv;
pub use message_recv::*;

mod message_send;
pub use message_send::*;

mod node;
pub use node::*;

mod node_metrics;
pub use node_metrics::*;

pub mod node_state;

mod peer_connection;
pub use peer_connection::*;

pub mod peer_connection_state;

mod peer_feature_set;
pub use peer_feature_set::*;

mod peer_hello;
pub use peer_hello::*;

mod peer_protocol;
pub use peer_protocol::*;

mod presets;
pub use presets::*;

mod topic;
pub use topic::*;

mod topic_subscription;
pub use topic_subscription::*;

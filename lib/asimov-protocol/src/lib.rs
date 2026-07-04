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

mod node;
pub use node::*;

mod gossip_protocol;
pub use gossip_protocol::*;

mod ping_protocol;
pub use ping_protocol::*;

mod presets;
pub use presets::*;

mod topic;
pub use topic::*;

mod topic_subscription;
pub use topic_subscription::*;

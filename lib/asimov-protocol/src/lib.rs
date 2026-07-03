// This is free and unencumbered software released into the public domain.

#![no_std]
#![forbid(unsafe_code)]
#![cfg_attr(docsrs, feature(doc_cfg))]

extern crate alloc;

#[cfg(feature = "std")]
extern crate std;

#[doc(hidden)]
pub use iroh;

pub use iroh::{
    Endpoint, EndpointAddr, EndpointId,
    endpoint::presets::Preset,
    protocol::{Router, RouterBuilder},
};
pub use iroh_tickets::{ParseError as TicketParsingError, Ticket, endpoint::EndpointTicket};

mod ping_protocol;
pub use ping_protocol::*;

mod presets;
pub use presets::*;

// This is free and unencumbered software released into the public domain.

use super::NodeProtocol;
use crate::{Endpoint, GossipProtocol, Router};
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
    pub(crate) node: NodeProtocol,
    pub(crate) gossip: GossipProtocol,
    pub(crate) peers: Vec<EndpointId>,
}

pub struct Terminating {
    pub(crate) router: Router,
}

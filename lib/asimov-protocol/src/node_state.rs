// This is free and unencumbered software released into the public domain.

#![allow(dead_code)]

use super::NodeProtocol;
use crate::{Endpoint, GossipProtocol, Router};
use alloc::vec::Vec;
use iroh::{EndpointId, endpoint::Builder as EndpointBuilder};

#[derive(Debug)]
pub struct Building {
    pub(crate) endpoint: EndpointBuilder,
}

#[derive(Debug)]
pub struct Bound {
    pub(crate) endpoint: Endpoint,
}

#[derive(Debug)]
pub struct Running {
    pub(crate) router: Router,
    pub(crate) node: NodeProtocol,
    pub(crate) gossip: GossipProtocol,
    pub(crate) peers: Vec<EndpointId>,
}

#[derive(Debug)]
pub struct Terminating {
    pub(crate) router: Router,
}

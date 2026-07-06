// This is free and unencumbered software released into the public domain.

#![allow(dead_code)]

use crate::PeerHello;
use iroh::endpoint::{Connection, RecvStream, SendStream};

/// The transport layer (QUIC) has completed its handshake.
#[derive(Debug)]
pub struct Connected {
    pub(crate) inner: Connection,
}

/// The hello request has been sent.
#[derive(Debug)]
pub struct Negotiating {
    pub(crate) inner: Connection,
    pub(crate) send: SendStream,
    pub(crate) recv: RecvStream,
}

/// The hello response has been received.
#[derive(Debug)]
pub struct Ready {
    pub(crate) inner: Connection,
    pub(crate) send: SendStream,
    pub(crate) recv: RecvStream,
    pub(crate) hello: PeerHello,
}

/// The connection has been closed.
#[derive(Debug)]
pub struct Closed {
    pub(crate) inner: Connection,
}

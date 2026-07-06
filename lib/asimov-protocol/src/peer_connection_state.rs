// This is free and unencumbered software released into the public domain.

#![allow(dead_code)]

use iroh::endpoint::{Connection, RecvStream, SendStream};

use crate::PeerHello;

/// The transport layer (QUIC) has completed its handshake.
pub struct Connected {
    pub(crate) inner: Connection,
}

/// The hello request has been sent.
pub struct Negotiating {
    pub(crate) inner: Connection,
    pub(crate) send: SendStream,
    pub(crate) recv: RecvStream,
}

/// The hello response has been received.
pub struct Ready {
    pub(crate) inner: Connection,
    pub(crate) send: SendStream,
    pub(crate) recv: RecvStream,
    pub(crate) hello: PeerHello,
}

/// The connection has been closed.
pub struct Closed {
    pub(crate) inner: Connection,
}

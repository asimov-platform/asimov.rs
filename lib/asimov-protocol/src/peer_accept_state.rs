// This is free and unencumbered software released into the public domain.

#![allow(dead_code)]

use crate::PeerHello;
use iroh::endpoint::{Connection, RecvStream, SendStream};

/// The transport layer (QUIC) has completed its handshake.
#[derive(Debug)]
pub struct Accepted {
    pub(crate) inner: Connection,
}

/// The hello request is expected.
#[derive(Debug)]
pub struct HelloExpected {
    pub(crate) inner: Connection,
    pub(crate) send: SendStream,
    pub(crate) recv: RecvStream,
}

/// The hello request has been received.
#[derive(Debug)]
pub struct HelloReceived {
    pub(crate) inner: Connection,
    pub(crate) send: SendStream,
    pub(crate) recv: RecvStream,
    pub(crate) hello: PeerHello,
}

/// The hello response has been sent.
#[derive(Debug)]
pub struct HelloSent {
    pub(crate) inner: Connection,
    pub(crate) send: SendStream,
    pub(crate) recv: RecvStream,
    pub(crate) hello: PeerHello,
}

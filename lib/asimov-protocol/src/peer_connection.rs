// This is free and unencumbered software released into the public domain.

#![allow(dead_code)]

use crate::{
    ConnectError, Message, MessageRecv, MessageSend, PeerHello, RecvError, SendError,
    peer_connection_state::*,
};
use core::convert::Infallible;

#[derive(Debug)]
pub struct PeerConnection<State = Connected>(State);

impl From<iroh::endpoint::Connection> for PeerConnection {
    fn from(inner: iroh::endpoint::Connection) -> Self {
        Self(Connected { inner })
    }
}

impl PeerConnection<Connected> {
    pub async fn send_hello(self) -> Result<PeerConnection<Negotiating>, ConnectError> {
        let Connected { inner } = self.0;
        let (send, recv) = inner.open_bi().await?;
        let mut connection = PeerConnection(Negotiating { inner, send, recv });
        let request = Message::Hello(PeerHello::default());
        let _ = connection.send(request).await.unwrap();
        Ok(connection)
    }
}

impl PeerConnection<Negotiating> {
    pub async fn recv_hello(mut self) -> Result<PeerConnection<Ready>, ConnectError> {
        let response = self.recv().await.unwrap();
        let Message::Hello(hello) = response else {
            return Err(ConnectError::InvalidResponse);
        };
        let Negotiating { inner, send, recv } = self.0;
        Ok(PeerConnection(Ready {
            inner,
            send,
            recv,
            hello,
        }))
    }
}

impl PeerConnection<Ready> {
    pub fn close(self) -> Result<PeerConnection<Closed>, Infallible> {
        let Ready { inner, .. } = self.0;
        inner.close(0u32.into(), &[]);
        Ok(PeerConnection(Closed { inner }))
    }
}

impl MessageSend for PeerConnection<Negotiating> {
    async fn write_all(&mut self, buffer: &[u8]) -> Result<(), SendError> {
        Ok(self.0.send.write_all(buffer).await?)
    }
}

impl MessageSend for PeerConnection<Ready> {
    async fn write_all(&mut self, buffer: &[u8]) -> Result<(), SendError> {
        Ok(self.0.send.write_all(buffer).await?)
    }
}

impl MessageRecv for PeerConnection<Negotiating> {
    async fn read_exact(&mut self, buffer: &mut [u8]) -> Result<(), RecvError> {
        Ok(self.0.recv.read_exact(buffer).await?)
    }
}

impl MessageRecv for PeerConnection<Ready> {
    async fn read_exact(&mut self, buffer: &mut [u8]) -> Result<(), RecvError> {
        Ok(self.0.recv.read_exact(buffer).await?)
    }
}

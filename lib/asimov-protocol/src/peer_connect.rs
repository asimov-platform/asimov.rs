// This is free and unencumbered software released into the public domain.

#![allow(dead_code)]

use crate::{
    ConnectError, Message, MessageRecv, MessageSend, PeerConnection, PeerHello, RecvError,
    SendError, peer_connection_state::*,
};
use core::convert::Infallible;

#[derive(Debug)]
pub struct PeerConnect<State = Connected>(State);

impl From<iroh::endpoint::Connection> for PeerConnect {
    fn from(inner: iroh::endpoint::Connection) -> Self {
        Self(Connected { inner })
    }
}

impl PeerConnect<Connected> {
    pub async fn send_hello(self) -> Result<PeerConnect<Negotiating>, ConnectError> {
        let Connected { inner } = self.0;
        let (send, recv) = inner.open_bi().await?;
        let mut connection = PeerConnect(Negotiating { inner, send, recv });
        let request = Message::Hello(PeerHello::default());
        let _ = connection.send(request).await?;
        Ok(connection)
    }
}

impl PeerConnect<Negotiating> {
    pub async fn recv_hello(mut self) -> Result<PeerConnect<Ready>, ConnectError> {
        let response = self.recv().await?;
        let Message::Hello(hello) = response else {
            return Err(ConnectError::InvalidMessage(response));
        };
        let Negotiating { inner, send, recv } = self.0;
        Ok(PeerConnect(Ready {
            inner,
            send,
            recv,
            hello,
        }))
    }
}

impl PeerConnect<Ready> {
    pub fn hello(&self) -> &PeerHello {
        return &self.0.hello;
    }

    pub fn close(self) -> Result<PeerConnect<Closed>, Infallible> {
        let Ready { inner, .. } = self.0;
        inner.close(0u32.into(), &[]);
        Ok(PeerConnect(Closed { inner }))
    }

    pub fn into_connection(self) -> PeerConnection {
        let Ready {
            inner,
            send,
            recv,
            hello,
        } = self.0;
        PeerConnection {
            inner,
            send,
            recv,
            hello,
        }
    }
}

impl MessageSend for PeerConnect<Negotiating> {
    async fn write_all(&mut self, buffer: &[u8]) -> Result<(), SendError> {
        Ok(self.0.send.write_all(buffer).await?)
    }
}

impl MessageSend for PeerConnect<Ready> {
    async fn write_all(&mut self, buffer: &[u8]) -> Result<(), SendError> {
        Ok(self.0.send.write_all(buffer).await?)
    }
}

impl MessageRecv for PeerConnect<Negotiating> {
    async fn read_exact(&mut self, buffer: &mut [u8]) -> Result<(), RecvError> {
        Ok(self.0.recv.read_exact(buffer).await?)
    }
}

impl MessageRecv for PeerConnect<Ready> {
    async fn read_exact(&mut self, buffer: &mut [u8]) -> Result<(), RecvError> {
        Ok(self.0.recv.read_exact(buffer).await?)
    }
}

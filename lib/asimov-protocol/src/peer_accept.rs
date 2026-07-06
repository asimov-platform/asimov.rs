// This is free and unencumbered software released into the public domain.

#![allow(dead_code)]

use crate::{
    AcceptError, Message, MessageRecv, MessageSend, PeerConnection, PeerHello, RecvError,
    SendError, peer_accept_state::*,
};

#[derive(Debug)]
pub struct PeerAccept<State = Accepted>(State);

impl From<iroh::endpoint::Connection> for PeerAccept {
    fn from(inner: iroh::endpoint::Connection) -> Self {
        Self(Accepted { inner })
    }
}

impl PeerAccept<Accepted> {
    pub async fn recv_hello(self) -> Result<PeerAccept<HelloReceived>, AcceptError> {
        let Accepted { inner } = self.0;
        let (send, recv) = inner.accept_bi().await?;
        let connection = PeerAccept(HelloExpected { inner, send, recv });
        Ok(connection.recv_hello().await?)
    }
}

impl PeerAccept<HelloExpected> {
    pub async fn recv_hello(mut self) -> Result<PeerAccept<HelloReceived>, AcceptError> {
        let response = self.recv().await?;
        let HelloExpected { inner, send, recv } = self.0;
        let Message::Hello(hello) = response else {
            return Err(AcceptError::InvalidMessage(response));
        };
        Ok(PeerAccept(HelloReceived {
            inner,
            send,
            recv,
            hello,
        }))
    }
}

impl PeerAccept<HelloReceived> {
    pub async fn send_hello(mut self) -> Result<PeerAccept<HelloSent>, AcceptError> {
        let HelloReceived { hello, .. } = &self.0;
        let request = Message::Hello(hello.clone()); // TODO
        let _ = self.send(request).await?;
        let HelloReceived {
            inner,
            send,
            recv,
            hello,
        } = self.0;
        Ok(PeerAccept(HelloSent {
            inner,
            send,
            recv,
            hello,
        }))
    }
}

impl PeerAccept<HelloSent> {
    pub fn hello(&self) -> &PeerHello {
        return &self.0.hello;
    }

    pub fn into_connection(self) -> PeerConnection {
        let HelloSent {
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

impl MessageRecv for PeerAccept<HelloExpected> {
    async fn read_exact(&mut self, buffer: &mut [u8]) -> Result<(), RecvError> {
        Ok(self.0.recv.read_exact(buffer).await?)
    }
}

impl MessageRecv for PeerAccept<HelloReceived> {
    async fn read_exact(&mut self, buffer: &mut [u8]) -> Result<(), RecvError> {
        Ok(self.0.recv.read_exact(buffer).await?)
    }
}

impl MessageRecv for PeerAccept<HelloSent> {
    async fn read_exact(&mut self, buffer: &mut [u8]) -> Result<(), RecvError> {
        Ok(self.0.recv.read_exact(buffer).await?)
    }
}

impl MessageSend for PeerAccept<HelloReceived> {
    async fn write_all(&mut self, buffer: &[u8]) -> Result<(), SendError> {
        Ok(self.0.send.write_all(buffer).await?)
    }
}

impl MessageSend for PeerAccept<HelloSent> {
    async fn write_all(&mut self, buffer: &[u8]) -> Result<(), SendError> {
        Ok(self.0.send.write_all(buffer).await?)
    }
}

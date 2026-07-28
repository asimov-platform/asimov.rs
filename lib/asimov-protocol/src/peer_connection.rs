// This is free and unencumbered software released into the public domain.

#![allow(dead_code)]

use crate::{Message, MessageRecv, MessageSend, PeerHello, PingError, RecvError, SendError};
use iroh::endpoint::{Connection, RecvStream, SendStream};
use tokio::time::{Duration, Instant};

#[derive(Debug)]
pub struct PeerConnection {
    pub(crate) inner: Connection,
    pub(crate) send: SendStream,
    pub(crate) recv: RecvStream,
    pub(crate) hello: PeerHello,
}

impl PeerConnection {
    pub fn hello(&self) -> &PeerHello {
        return &self.hello;
    }

    pub async fn ping(&mut self) -> Result<Duration, PingError> {
        // Begin measuring elapsed time:
        let start = Instant::now();

        // Send the ping request:
        let _ = self.send(Message::Ping).await?;

        // Read the ping response:
        let response = self.recv().await?;
        assert_eq!(response, Message::Ping);

        // Measure the duration of this interaction:
        let duration = start.elapsed();

        Ok(duration)
    }

    // pub fn close(self) -> Result<PeerConnection<Closed>, Infallible> {
    //     let Ready { inner, .. } = self.0;
    //     inner.close(0u32.into(), &[]);
    //     Ok(PeerConnection(Closed { inner }))
    // }
}

impl MessageSend for PeerConnection {
    async fn write_all(&mut self, buffer: &[u8]) -> Result<(), SendError> {
        Ok(self.send.write_all(buffer).await?)
    }
}

impl MessageRecv for PeerConnection {
    async fn read_exact(&mut self, buffer: &mut [u8]) -> Result<(), RecvError> {
        Ok(self.recv.read_exact(buffer).await?)
    }
}

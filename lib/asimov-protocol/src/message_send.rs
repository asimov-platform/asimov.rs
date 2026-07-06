// This is free and unencumbered software released into the public domain.

use crate::{MESSAGE_HEADER_LEN, MessageLen, Message, SendError};

pub trait MessageSend {
    fn send(&mut self, request: Message) -> impl Future<Output = Result<usize, SendError>> {
        async move {
            let mut buffer = [0u8; 64];
            let (head_buffer, body_buffer) = buffer.split_at_mut(MESSAGE_HEADER_LEN);

            let body: &mut [u8] = postcard::to_slice(&request, body_buffer)?;

            let body_len = body.len() as MessageLen;
            assert!(body_len <= MessageLen::MAX);
            head_buffer.copy_from_slice(&body_len.to_be_bytes());

            let write_len = MESSAGE_HEADER_LEN + body_len as usize;
            self.write_all(&buffer[..write_len]).await?;

            Ok(write_len)
        }
    }

    fn write_all(&mut self, buffer: &[u8]) -> impl Future<Output = Result<(), SendError>>;
}

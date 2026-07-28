// This is free and unencumbered software released into the public domain.

use crate::{Message, RecvError};
use heapless::Vec;

pub trait MessageRecv {
    fn recv(&mut self) -> impl Future<Output = Result<Message, RecvError>> {
        async {
            type Len = u32;
            const HEAD_LEN: usize = size_of::<Len>();

            let mut head_buffer = [0u8; HEAD_LEN];
            self.read_exact(&mut head_buffer).await?;

            let body_len = u32::from_be_bytes(head_buffer) as usize;

            let mut buffer: Vec<u8, 1024> = Vec::new();
            assert!(body_len <= buffer.capacity());
            buffer.resize(body_len, 0)?;

            self.read_exact(buffer.as_mut_slice()).await?;

            let response: Message = postcard::from_bytes(buffer.as_slice())?;

            Ok(response)
        }
    }

    fn read_exact(&mut self, buffer: &mut [u8]) -> impl Future<Output = Result<(), RecvError>>;
}

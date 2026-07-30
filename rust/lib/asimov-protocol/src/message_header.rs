// This is free and unencumbered software released into the public domain.

pub type MessageLen = u32;

pub const MESSAGE_HEADER_LEN: usize = size_of::<MessageLen>();

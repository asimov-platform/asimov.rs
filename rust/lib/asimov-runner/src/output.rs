// This is free and unencumbered software released into the public domain.

use alloc::boxed::Box;
use derive_more::Debug;
use tokio::io::AsyncWrite;

pub type AnyOutput = Output;
pub type GraphOutput = Output;
pub type NoOutput = ();
pub type QueryOutput = Output;
pub type TextOutput = Output;

#[derive(Debug)]
pub enum Output {
    Ignored,
    Inherited,
    Captured,
    AsyncWrite(#[debug(skip)] Box<dyn AsyncWrite + Send + Sync + Unpin>),
}

impl Output {
    #[cfg(feature = "std")]
    pub fn as_stdio(&self) -> std::process::Stdio {
        use std::process::Stdio;
        match self {
            Output::Ignored => Stdio::null(),
            Output::Inherited => Stdio::inherit(),
            Output::Captured => Stdio::piped(),
            Output::AsyncWrite(_) => Stdio::piped(),
        }
    }
}

#[cfg(feature = "std")]
impl Into<std::process::Stdio> for Output {
    fn into(self) -> std::process::Stdio {
        use std::process::Stdio;
        match self {
            Output::Ignored => Stdio::null(),
            Output::Inherited => Stdio::inherit(),
            Output::Captured => Stdio::piped(),
            Output::AsyncWrite(_) => Stdio::piped(),
        }
    }
}

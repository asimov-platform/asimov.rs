// This is free and unencumbered software released into the public domain.

use derive_more::Debug;
use std::process::Stdio;
use tokio::io::AsyncWrite;

pub type AnyOutput = Output;
pub type GraphOutput = Output;
pub type NoOutput = ();
pub type QueryOutput = Output;
pub type TextOutput = Output;

#[derive(Debug)]
pub enum Output {
    Ignored,
    Captured,
    AsyncWrite(#[debug(skip)] Box<dyn AsyncWrite + Send + Sync>),
}

impl Output {
    pub fn as_stdio(&self) -> Stdio {
        match self {
            Output::Ignored => Stdio::null(),
            Output::Captured => Stdio::piped(),
            Output::AsyncWrite(_) => Stdio::piped(),
        }
    }
}

impl Into<Stdio> for Output {
    fn into(self) -> Stdio {
        match self {
            Output::Ignored => Stdio::null(),
            Output::Captured => Stdio::piped(),
            Output::AsyncWrite(_) => Stdio::piped(),
        }
    }
}

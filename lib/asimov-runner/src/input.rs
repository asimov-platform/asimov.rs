// This is free and unencumbered software released into the public domain.

use derive_more::Debug;
use std::process::Stdio;
use tokio::io::AsyncRead;

pub type AnyInput = Input;
pub type EmptyInput = ();
pub type GraphInput = Input;
pub type QueryInput = Input;
pub type TextInput = Input;

#[derive(Debug)]
pub enum Input {
    Ignored,
    AsyncRead(#[debug(skip)] Box<dyn AsyncRead + Send + Sync>),
}

impl Input {
    pub fn as_stdio(&self) -> Stdio {
        match self {
            Input::Ignored => Stdio::null(),
            Input::AsyncRead(_) => Stdio::piped(),
        }
    }
}

impl Into<Stdio> for Input {
    fn into(self) -> Stdio {
        match self {
            Input::Ignored => Stdio::null(),
            Input::AsyncRead(_) => Stdio::piped(),
        }
    }
}

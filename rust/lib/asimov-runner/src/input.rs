// This is free and unencumbered software released into the public domain.

use alloc::boxed::Box;
use derive_more::Debug;
use tokio::io::AsyncRead;

pub type AnyInput = Input;
pub type GraphInput = Input;
pub type NoInput = ();
pub type QueryInput = Input;
pub type TextInput = Input;

#[derive(Debug)]
pub enum Input {
    Ignored,
    AsyncRead(#[debug(skip)] Box<dyn AsyncRead + Send + Sync + Unpin>),
}

impl Input {
    #[cfg(feature = "std")]
    pub fn as_stdio(&self) -> std::process::Stdio {
        use std::process::Stdio;
        match self {
            Input::Ignored => Stdio::null(),
            Input::AsyncRead(_) => Stdio::piped(),
        }
    }
}

#[cfg(feature = "std")]
impl Into<std::process::Stdio> for Input {
    fn into(self) -> std::process::Stdio {
        use std::process::Stdio;
        match self {
            Input::Ignored => Stdio::null(),
            Input::AsyncRead(_) => Stdio::piped(),
        }
    }
}

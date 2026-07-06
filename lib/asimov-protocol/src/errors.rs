// This is free and unencumbered software released into the public domain.

use alloc::boxed::Box;
use core::error::Error;
use known_errors::sysexits::SysexitsError;
use thiserror::Error;

#[derive(Debug, Error)]
#[non_exhaustive]
pub enum BindError {
    #[error(transparent)]
    Other(#[from] iroh::endpoint::BindError),
}

impl From<BindError> for SysexitsError {
    fn from(_: BindError) -> Self {
        SysexitsError::EX_SOFTWARE // TODO
    }
}

#[derive(Debug, Error)]
#[non_exhaustive]
pub enum ConnectError {
    #[error("invalid response from peer")]
    InvalidResponse,

    #[error(transparent)]
    Postcard(#[from] postcard::Error),

    #[error(transparent)]
    Iroh(#[from] iroh::endpoint::ConnectError),

    #[error(transparent)]
    NoqConnect(#[from] iroh::endpoint::ConnectionError),

    #[error(transparent)]
    NoqWrite(#[from] iroh::endpoint::WriteError),
}

impl From<ConnectError> for SysexitsError {
    fn from(_: ConnectError) -> Self {
        SysexitsError::EX_SOFTWARE // TODO
    }
}

#[derive(Debug, Error)]
#[non_exhaustive]
pub enum PingError {
    #[error(transparent)]
    Other(#[from] Box<dyn Error>),
}

impl From<PingError> for SysexitsError {
    fn from(_: PingError) -> Self {
        SysexitsError::EX_SOFTWARE // TODO
    }
}

#[derive(Debug, Error)]
#[non_exhaustive]
pub enum PublishError {
    #[error(transparent)]
    Gossip(#[from] iroh_gossip::api::ApiError),

    #[error(transparent)]
    Other(#[from] Box<dyn Error>),
}

impl From<PublishError> for SysexitsError {
    fn from(_: PublishError) -> Self {
        SysexitsError::EX_SOFTWARE // TODO
    }
}

#[derive(Debug, Error)]
#[non_exhaustive]
pub enum StartError {
    #[error(transparent)]
    Other(#[from] Box<dyn Error>),
}

impl From<StartError> for SysexitsError {
    fn from(_: StartError) -> Self {
        SysexitsError::EX_SOFTWARE // TODO
    }
}

#[derive(Debug, Error)]
#[non_exhaustive]
pub enum SubscribeError {
    #[error(transparent)]
    Gossip(#[from] iroh_gossip::api::ApiError),

    #[error(transparent)]
    Other(#[from] Box<dyn Error>),
}

impl From<SubscribeError> for SysexitsError {
    fn from(_: SubscribeError) -> Self {
        SysexitsError::EX_SOFTWARE // TODO
    }
}

#[derive(Debug, Error)]
#[non_exhaustive]
pub enum TerminateError {
    #[error(transparent)]
    Other(#[from] tokio::task::JoinError),
}

impl From<TerminateError> for SysexitsError {
    fn from(_: TerminateError) -> Self {
        SysexitsError::EX_SOFTWARE // TODO
    }
}

#[derive(Debug, Error)]
#[non_exhaustive]
pub enum SendError {
    #[error(transparent)]
    Postcard(#[from] postcard::Error),

    #[error(transparent)]
    Other(#[from] iroh::endpoint::WriteError),
}

impl From<SendError> for SysexitsError {
    fn from(_: SendError) -> Self {
        SysexitsError::EX_SOFTWARE // TODO
    }
}

#[derive(Debug, Error)]
#[non_exhaustive]
pub enum RecvError {
    #[error(transparent)]
    Postcard(#[from] postcard::Error),

    #[error(transparent)]
    Other(#[from] iroh::endpoint::ReadExactError),
}

impl From<RecvError> for SysexitsError {
    fn from(_: RecvError) -> Self {
        SysexitsError::EX_SOFTWARE // TODO
    }
}

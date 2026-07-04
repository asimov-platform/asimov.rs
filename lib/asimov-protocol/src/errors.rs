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

// This is free and unencumbered software released into the public domain.

use alloc::boxed::Box;
use async_trait::async_trait;
use core::result::Result;

/// Asynchronous execution with error handling.
#[async_trait]
pub trait Execute<T, E> {
    async fn execute(&mut self) -> Result<T, E>;
}

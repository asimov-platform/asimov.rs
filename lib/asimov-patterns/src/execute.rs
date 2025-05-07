// This is free and unencumbered software released into the public domain.

use async_trait::async_trait;
use dogma::prelude::{Box, Result};

/// Asynchronous execution with error handling.
#[async_trait]
pub trait Execute<T, E> {
    async fn execute(&mut self) -> Result<T, E>;
}

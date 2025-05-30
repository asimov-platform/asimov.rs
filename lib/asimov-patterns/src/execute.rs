// This is free and unencumbered software released into the public domain.

#![allow(unused_imports)]

use async_trait::async_trait;
use dogma::prelude::{Box, Result};

/// Asynchronous execution with error handling.
#[async_trait]
pub trait Execute<T, E> {
    async fn execute(&mut self) -> Result<T, E>;
}

mod adapter;
pub use adapter::*;

mod cataloger;
pub use cataloger::*;

mod emitter;
pub use emitter::*;

mod fetcher;
pub use fetcher::*;

mod importer;
pub use importer::*;

mod prompter;
pub use prompter::*;

mod provider;
pub use provider::*;

mod reader;
pub use reader::*;

mod reasoner;
pub use reasoner::*;

mod registrar;
pub use registrar::*;

mod runner;
pub use runner::*;

mod searcher;
pub use searcher::*;

mod writer;
pub use writer::*;

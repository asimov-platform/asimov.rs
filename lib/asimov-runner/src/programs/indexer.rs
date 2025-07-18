// This is free and unencumbered software released into the public domain.

use crate::{Executor, ExecutorError, GraphInput, NoOutput};
use async_trait::async_trait;
use derive_more::Debug;
use std::{
    ffi::OsStr,
    io::{Cursor, Read},
    process::Stdio,
};
use tokio::io::{AsyncRead, AsyncWrite};

pub use asimov_patterns::IndexerOptions;

/// See: https://asimov-specs.github.io/program-patterns/#indexer
pub type IndexerResult = std::result::Result<(), ExecutorError>;

/// See: https://asimov-specs.github.io/program-patterns/#indexer
#[allow(unused)]
#[derive(Debug)]
pub struct Indexer {
    executor: Executor,
    options: IndexerOptions,
    input: GraphInput,
    output: NoOutput,
}

impl Indexer {
    pub fn new(program: impl AsRef<OsStr>, input: GraphInput, options: IndexerOptions) -> Self {
        let mut executor = Executor::new(program);
        executor
            .command()
            .stdin(input.as_stdio())
            .stdout(Stdio::null())
            .stderr(Stdio::piped());

        Self {
            executor,
            options,
            input,
            output: (),
        }
    }
}

impl asimov_patterns::Indexer<ExecutorError> for Indexer {}

#[async_trait]
impl asimov_patterns::Execute<(), ExecutorError> for Indexer {
    async fn execute(&mut self) -> IndexerResult {
        let _stdout = self.executor.execute_with_input(&mut self.input).await?;
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use asimov_patterns::Execute;

    #[tokio::test]
    async fn test_execute() {
        // TODO
    }
}

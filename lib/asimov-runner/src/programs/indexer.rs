// This is free and unencumbered software released into the public domain.

use crate::{Executor, ExecutorError, GraphInput, NoOutput};
use alloc::{boxed::Box, format, vec};
use async_trait::async_trait;
use derive_more::Debug;
use std::{ffi::OsStr, process::Stdio};

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
            .args(if let Some(ref input) = options.input {
                vec![format!("--input={}", input)]
            } else {
                vec![]
            })
            .args(&options.other)
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

    pub async fn execute(&mut self) -> IndexerResult {
        let _stdout = self.executor.execute_with_input(&mut self.input).await?;
        Ok(())
    }
}

impl asimov_patterns::Indexer<ExecutorError> for Indexer {}

#[async_trait]
impl asimov_patterns::Execute<(), ExecutorError> for Indexer {
    async fn execute(&mut self) -> IndexerResult {
        self.execute().await
    }
}

#[cfg(test)]
mod tests {
    //use super::*;
    //use asimov_patterns::Execute;

    #[tokio::test]
    async fn test_execute() {
        // TODO
    }
}

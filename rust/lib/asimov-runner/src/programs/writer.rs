// This is free and unencumbered software released into the public domain.

use crate::{AnyOutput, Executor, ExecutorError, GraphInput};
use alloc::{boxed::Box, format, vec, vec::Vec};
use async_trait::async_trait;
use derive_more::Debug;
use std::{ffi::OsStr, io::Cursor, process::Stdio};

pub use asimov_patterns::WriterOptions;

/// See: https://asimov-specs.github.io/program-patterns/#writer
pub type WriterResult = std::result::Result<Cursor<Vec<u8>>, ExecutorError>; // TODO

/// See: https://asimov-specs.github.io/program-patterns/#writer
#[allow(unused)]
#[derive(Debug)]
pub struct Writer {
    executor: Executor,
    options: WriterOptions,
    input: GraphInput,
    output: AnyOutput,
}

impl Writer {
    pub fn new(
        program: impl AsRef<OsStr>,
        input: GraphInput,
        output: AnyOutput,
        options: WriterOptions,
    ) -> Self {
        let mut executor = Executor::new(program);
        executor
            .command()
            .args(if let Some(ref input) = options.input {
                vec![format!("--input={}", input)]
            } else {
                vec![]
            })
            .args(if let Some(ref output) = options.output {
                vec![format!("--output={}", output)]
            } else {
                vec![]
            })
            .args(&options.other)
            .stdin(input.as_stdio())
            .stdout(output.as_stdio())
            .stderr(Stdio::piped());

        Self {
            executor,
            options,
            input,
            output,
        }
    }

    pub async fn execute(&mut self) -> WriterResult {
        let stdout = self.executor.execute_with_input(&mut self.input).await?;
        Ok(stdout)
    }
}

impl asimov_patterns::Writer<Cursor<Vec<u8>>, ExecutorError> for Writer {}

#[async_trait]
impl asimov_patterns::Execute<Cursor<Vec<u8>>, ExecutorError> for Writer {
    async fn execute(&mut self) -> WriterResult {
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

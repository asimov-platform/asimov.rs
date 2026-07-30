// This is free and unencumbered software released into the public domain.

use crate::{Executor, ExecutorError, Input, Output};
use alloc::{boxed::Box, format, vec::Vec};
use async_trait::async_trait;
use derive_more::Debug;
use std::{ffi::OsStr, io::Cursor, process::Stdio};

pub use asimov_patterns::RunnerOptions;

/// See: https://asimov-specs.github.io/program-patterns/#runner
pub type RunnerResult = std::result::Result<Cursor<Vec<u8>>, ExecutorError>; // TODO

/// See: https://asimov-specs.github.io/program-patterns/#runner
#[allow(unused)]
#[derive(Debug)]
pub struct Runner {
    executor: Executor,
    options: RunnerOptions,
    input: Input,
    output: Output,
}

impl Runner {
    pub fn new(
        program: impl AsRef<OsStr>,
        input: Input,
        output: Output,
        options: RunnerOptions,
    ) -> Self {
        let mut executor = Executor::new(program);
        executor
            .command()
            .args(
                &options
                    .define
                    .iter()
                    .map(|(k, v)| format!("--define={}={}", k, v))
                    .collect::<Vec<_>>(),
            )
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

    pub async fn execute(&mut self) -> RunnerResult {
        let stdout = self.executor.execute_with_input(&mut self.input).await?;
        Ok(stdout)
    }
}

impl asimov_patterns::Runner<Cursor<Vec<u8>>, ExecutorError> for Runner {}

#[async_trait]
impl asimov_patterns::Execute<Cursor<Vec<u8>>, ExecutorError> for Runner {
    async fn execute(&mut self) -> RunnerResult {
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

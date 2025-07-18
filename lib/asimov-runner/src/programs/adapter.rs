// This is free and unencumbered software released into the public domain.

use crate::{Executor, ExecutorError, GraphOutput, QueryInput};
use async_trait::async_trait;
use derive_more::Debug;
use std::{
    ffi::OsStr,
    io::{Cursor, Read},
    process::Stdio,
};
use tokio::io::{AsyncRead, AsyncWrite};

pub use asimov_patterns::AdapterOptions;

/// See: https://asimov-specs.github.io/program-patterns/#adapter
pub type AdapterResult = std::result::Result<Cursor<Vec<u8>>, ExecutorError>; // TODO

/// See: https://asimov-specs.github.io/program-patterns/#adapter
#[allow(unused)]
#[derive(Debug)]
pub struct Adapter {
    executor: Executor,
    options: AdapterOptions,
    input: QueryInput,
    output: GraphOutput,
}

impl Adapter {
    pub fn new(
        program: impl AsRef<OsStr>,
        input: QueryInput,
        output: GraphOutput,
        options: AdapterOptions,
    ) -> Self {
        let mut executor = Executor::new(program);
        executor
            .command()
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

    pub async fn execute(&mut self) -> AdapterResult {
        let stdout = self.executor.execute_with_input(&mut self.input).await?;
        Ok(stdout)
    }
}

impl asimov_patterns::Adapter<Cursor<Vec<u8>>, ExecutorError> for Adapter {}

#[async_trait]
impl asimov_patterns::Execute<Cursor<Vec<u8>>, ExecutorError> for Adapter {
    async fn execute(&mut self) -> AdapterResult {
        self.execute().await
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

// This is free and unencumbered software released into the public domain.

use crate::{AnyInput, Executor, ExecutorError, GraphOutput};
use async_trait::async_trait;
use derive_more::Debug;
use std::{
    ffi::OsStr,
    io::{Cursor, Read},
    process::Stdio,
};
use tokio::io::{AsyncRead, AsyncWrite};

pub use asimov_patterns::ReaderOptions;

/// See: https://asimov-specs.github.io/program-patterns/#reader
pub type ReaderResult = std::result::Result<Cursor<Vec<u8>>, ExecutorError>; // TODO

/// See: https://asimov-specs.github.io/program-patterns/#reader
#[allow(unused)]
#[derive(Debug)]
pub struct Reader {
    executor: Executor,
    options: ReaderOptions,
    input: AnyInput,
    output: GraphOutput,
}

impl Reader {
    pub fn new(
        program: impl AsRef<OsStr>,
        input: AnyInput,
        output: GraphOutput,
        options: ReaderOptions,
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

    pub async fn execute(&mut self) -> ReaderResult {
        let stdout = self.executor.execute_with_input(&mut self.input).await?;
        Ok(stdout)
    }
}

impl asimov_patterns::Reader<Cursor<Vec<u8>>, ExecutorError> for Reader {}

#[async_trait]
impl asimov_patterns::Execute<Cursor<Vec<u8>>, ExecutorError> for Reader {
    async fn execute(&mut self) -> ReaderResult {
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

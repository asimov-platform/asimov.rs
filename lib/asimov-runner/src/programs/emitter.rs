// This is free and unencumbered software released into the public domain.

use crate::{Executor, ExecutorError, GraphOutput, Input, NoInput, Output};
use async_trait::async_trait;
use derive_more::Debug;
use std::{
    ffi::OsStr,
    io::{Cursor, Read},
    process::Stdio,
};
use tokio::io::{AsyncRead, AsyncWrite};

pub use asimov_patterns::EmitterOptions;

/// See: https://asimov-specs.github.io/program-patterns/#emitter
pub type EmitterResult = std::result::Result<Cursor<Vec<u8>>, ExecutorError>; // TODO

/// See: https://asimov-specs.github.io/program-patterns/#emitter
#[allow(unused)]
#[derive(Debug)]
pub struct Emitter {
    executor: Executor,
    options: EmitterOptions,
    input: NoInput,
    output: GraphOutput,
}

impl Emitter {
    pub fn new(program: impl AsRef<OsStr>, output: Output, options: EmitterOptions) -> Self {
        let mut executor = Executor::new(program);
        executor
            .command()
            .stdin(Stdio::null())
            .stdout(output.as_stdio())
            .stderr(Stdio::piped());

        Self {
            executor,
            options,
            input: (),
            output,
        }
    }
}

impl asimov_patterns::Emitter<Cursor<Vec<u8>>, ExecutorError> for Emitter {}

#[async_trait]
impl asimov_patterns::Execute<Cursor<Vec<u8>>, ExecutorError> for Emitter {
    async fn execute(&mut self) -> EmitterResult {
        let stdout = self.executor.execute().await?;
        Ok(stdout)
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

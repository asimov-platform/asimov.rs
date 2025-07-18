// This is free and unencumbered software released into the public domain.

use crate::{Executor, ExecutorError, GraphOutput, Input};
use async_trait::async_trait;
use derive_more::Debug;
use std::{
    ffi::OsStr,
    io::{Cursor, Read},
    process::Stdio,
};
use tokio::io::{AsyncRead, AsyncWrite};

pub use asimov_patterns::CatalogerOptions;

/// See: https://asimov-specs.github.io/program-patterns/#cataloger
pub type CatalogerResult = std::result::Result<Cursor<Vec<u8>>, ExecutorError>; // TODO

/// See: https://asimov-specs.github.io/program-patterns/#cataloger
#[allow(unused)]
#[derive(Debug)]
pub struct Cataloger {
    executor: Executor,
    options: CatalogerOptions,
    input: String,
    output: GraphOutput,
}

impl Cataloger {
    pub fn new(
        program: impl AsRef<OsStr>,
        input: impl AsRef<str>,
        output: GraphOutput,
        options: CatalogerOptions,
    ) -> Self {
        let mut executor = Executor::new(program);
        executor
            .command()
            .stdin(Stdio::null())
            .stdout(output.as_stdio())
            .stderr(Stdio::piped());

        Self {
            executor,
            options,
            input: input.as_ref().to_string(),
            output,
        }
    }
}

impl asimov_patterns::Cataloger<Cursor<Vec<u8>>, ExecutorError> for Cataloger {}

#[async_trait]
impl asimov_patterns::Execute<Cursor<Vec<u8>>, ExecutorError> for Cataloger {
    async fn execute(&mut self) -> CatalogerResult {
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

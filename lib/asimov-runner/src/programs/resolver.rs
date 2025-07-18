// This is free and unencumbered software released into the public domain.

use crate::{Executor, ExecutorError, Input, Output};
use async_trait::async_trait;
use derive_more::Debug;
use std::{
    ffi::OsStr,
    io::{Cursor, Read},
    process::Stdio,
};
use tokio::io::{AsyncBufReadExt, AsyncRead, AsyncWrite};

pub use asimov_patterns::ResolverOptions;

/// See: https://asimov-specs.github.io/program-patterns/#resolver
pub type ResolverResult = std::result::Result<Vec<String>, ExecutorError>;

/// See: https://asimov-specs.github.io/program-patterns/#resolver
#[allow(unused)]
#[derive(Debug)]
pub struct Resolver {
    executor: Executor,
    options: ResolverOptions,
    input: String,
    output: Output,
}

impl Resolver {
    pub fn new(
        program: impl AsRef<OsStr>,
        input: impl AsRef<str>,
        output: Output,
        options: ResolverOptions,
    ) -> Self {
        let input = input.as_ref().to_string();
        let mut executor = Executor::new(program);
        executor
            .command()
            .arg(&input)
            .stdin(Stdio::null())
            .stdout(output.as_stdio())
            .stderr(Stdio::piped());

        Self {
            executor,
            options,
            input,
            output,
        }
    }
}

impl asimov_patterns::Resolver<Vec<String>, ExecutorError> for Resolver {}

#[async_trait]
impl asimov_patterns::Execute<Vec<String>, ExecutorError> for Resolver {
    async fn execute(&mut self) -> ResolverResult {
        let _stdout = self.executor.execute().await?;
        //let lines = stdout.lines().into_iter().collect::<Vec<_>>().await?; // FIXME
        Ok(Vec::new())
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

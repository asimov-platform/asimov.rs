// This is free and unencumbered software released into the public domain.

use crate::{Runner, RunnerError};
use async_trait::async_trait;
use std::{ffi::OsStr, io::Cursor, process::Stdio};

pub use asimov_patterns::FetcherOptions;

pub type FetcherResult = std::result::Result<Cursor<Vec<u8>>, RunnerError>; // TODO

/// See: https://asimov-specs.github.io/program-patterns/#fetcher
#[derive(Debug)]
#[allow(unused)]
pub struct Fetcher {
    runner: Runner,
    options: FetcherOptions,
}

impl Fetcher {
    pub fn new(
        program: impl AsRef<OsStr>,
        input_url: impl AsRef<OsStr>,
        options: FetcherOptions,
    ) -> Self {
        let mut runner = Runner::new(program);

        runner
            .command()
            .arg(input_url)
            .stdin(Stdio::null())
            .stdout(Stdio::piped())
            .stderr(Stdio::piped());

        Self { runner, options }
    }
}

impl asimov_patterns::Fetcher<Cursor<Vec<u8>>, RunnerError> for Fetcher {}

#[async_trait]
impl asimov_patterns::Execute<Cursor<Vec<u8>>, RunnerError> for Fetcher {
    async fn execute(&mut self) -> FetcherResult {
        let stdout = self.runner.execute().await?;
        Ok(stdout)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use asimov_patterns::Execute;

    #[tokio::test]
    async fn test_execute() {
        let mut runner = Fetcher::new(
            "curl",
            "https://www.google.com/robots.txt",
            FetcherOptions::default(),
        );
        let result = runner.execute().await;
        assert!(result.is_ok());
    }
}

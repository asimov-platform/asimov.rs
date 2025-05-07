// This is free and unencumbered software released into the public domain.

use crate::{Runner, RunnerError};
use asimov_patterns::ImporterOptions;
use async_trait::async_trait;
use std::{ffi::OsStr, io::Cursor, process::Stdio};

pub type ImporterResult = std::result::Result<Cursor<Vec<u8>>, RunnerError>; // TODO

/// RDF dataset importer. Consumes a URL input, produces RDF output.
#[derive(Debug)]
#[allow(unused)]
pub struct Importer {
    runner: Runner,
    options: ImporterOptions,
}

impl Importer {
    pub fn new(program: impl AsRef<OsStr>, options: ImporterOptions) -> Self {
        let mut runner = Runner::new(program);

        runner
            .command()
            .arg(&options.input_url)
            .stdin(Stdio::null())
            .stdout(Stdio::piped())
            .stderr(Stdio::piped());

        Self { runner, options }
    }
}

impl asimov_patterns::Importer<Cursor<Vec<u8>>, RunnerError> for Importer {}

#[async_trait]
impl asimov_patterns::Execute<Cursor<Vec<u8>>, RunnerError> for Importer {
    async fn execute(&mut self) -> ImporterResult {
        let stdout = self.runner.execute().await?;
        Ok(stdout)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use asimov_patterns::Execute;

    #[tokio::test]
    async fn test_success() {
        let mut runner = Importer::new(
            "curl",
            ImporterOptions {
                input_url: "https://www.google.com/robots.txt".to_string(),
            },
        );
        let result = runner.execute().await;
        assert!(result.is_ok());
    }
}

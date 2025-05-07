// This is free and unencumbered software released into the public domain.

use asimov_patterns::FetcherOptions;
use std::{
    io::Result,
    path::{Path, PathBuf},
    process::{Output, Stdio},
};
use tokio::process::Command;

/// Network protocol fetcher. Consumes a URL input, produces some output.
#[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct Fetcher {
    program: PathBuf,
    options: FetcherOptions,
}

impl asimov_patterns::Fetcher for Fetcher {}

impl Fetcher {
    pub fn new(program: impl AsRef<Path>, options: FetcherOptions) -> Self {
        Self {
            program: program.as_ref().into(),
            options,
        }
    }

    pub async fn execute(&self) -> Result<Output> {
        let process = Command::new(&self.program)
            .arg(&self.options.input_url)
            .stdin(Stdio::null())
            .stdout(Stdio::piped())
            .stderr(Stdio::piped())
            .spawn()?;

        let output = process.wait_with_output().await?;
        tracing::trace!("The command exited with: {}", output.status);

        Ok(output)
    }
}

// This is free and unencumbered software released into the public domain.

use asimov_patterns::ImporterOptions;
use std::{
    io::Result,
    path::{Path, PathBuf},
    process::{Output, Stdio},
};
use tokio::process::Command;

/// RDF dataset importer. Consumes a URL input, produces RDF output.
#[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct Importer {
    program: PathBuf,
    options: ImporterOptions,
}

impl asimov_patterns::Importer for Importer {}

impl Importer {
    pub fn new(program: impl AsRef<Path>, options: ImporterOptions) -> Self {
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

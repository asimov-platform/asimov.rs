// This is free and unencumbered software released into the public domain.

use crate::{Runner, RunnerError};
use async_trait::async_trait;
use std::{ffi::OsStr, io::Read, process::Stdio};

pub use asimov_patterns::ProviderOptions;

pub type ProviderResult = std::result::Result<String, RunnerError>;

/// LLM inference provider. Consumes text input, produces text output.
#[derive(Debug)]
pub struct Provider {
    runner: Runner,
    #[allow(unused)]
    options: ProviderOptions,
}

impl Provider {
    pub fn new(program: impl AsRef<OsStr>, options: ProviderOptions) -> Self {
        let mut runner = Runner::new(program);

        runner
            .command()
            //.arg(&options.input_url)
            .stdin(Stdio::null())
            .stdout(Stdio::piped())
            .stderr(Stdio::piped());

        Self { runner, options }
    }
}

impl asimov_patterns::Provider<String, RunnerError> for Provider {}

#[async_trait]
impl asimov_patterns::Execute<String, RunnerError> for Provider {
    async fn execute(&mut self) -> ProviderResult {
        let mut stdout = self.runner.execute().await?;
        let mut result = String::new();
        stdout.read_to_string(&mut result)?;
        Ok(result)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use asimov_patterns::Execute;

    #[tokio::test]
    async fn test_success() {
        let mut runner = Provider::new(
            "cat",
            ProviderOptions {
                prompt: String::new(),
            },
        );
        let result = runner.execute().await;
        assert!(result.is_ok());
    }
}

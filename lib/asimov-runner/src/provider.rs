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
            .stdin(Stdio::piped())
            .stdout(Stdio::piped())
            .stderr(Stdio::piped());

        Self { runner, options }
    }
}

impl asimov_patterns::Provider<String, RunnerError> for Provider {}

#[async_trait]
impl asimov_patterns::Execute<String, RunnerError> for Provider {
    async fn execute(&mut self) -> ProviderResult {
        let mut process = self.runner.spawn().await?;

        let prompt = self.options.prompt.clone();
        let mut stdin = process.stdin.take().expect("Failed to capture stdin");
        tokio::spawn(async move {
            use tokio::io::AsyncWriteExt;
            stdin
                .write_all(prompt.as_bytes())
                .await
                .expect("Failed to write to stdin");
        });

        let mut stdout = self.runner.wait(process).await?;
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
    async fn test_execute() {
        let mut runner = Provider::new(
            "cat",
            ProviderOptions {
                prompt: "Hello, world!".into(),
            },
        );
        let result = runner.execute().await;
        assert!(result.is_ok());
        assert_eq!(result.unwrap(), String::from("Hello, world!"));
    }
}

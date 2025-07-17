// This is free and unencumbered software released into the public domain.

use crate::{Runner, RunnerError};
use async_trait::async_trait;
use std::{ffi::OsStr, io::Read, process::Stdio};

pub use asimov_patterns::PrompterOptions;
pub use asimov_prompt::{Prompt, PromptMessage, PromptRole};

pub type ProviderResult = std::result::Result<String, RunnerError>;

/// See: https://asimov-specs.github.io/program-patterns/#prompter
#[derive(Debug)]
pub struct Prompter {
    runner: Runner,
    prompt: Prompt,
    #[allow(unused)]
    options: PrompterOptions,
}

impl Prompter {
    pub fn new(program: impl AsRef<OsStr>, prompt: Prompt, options: PrompterOptions) -> Self {
        let mut runner = Runner::new(program);

        runner
            .command()
            .stdin(Stdio::piped())
            .stdout(Stdio::piped())
            .stderr(Stdio::piped());

        Self {
            runner,
            prompt,
            options,
        }
    }
}

impl asimov_patterns::Prompter<String, RunnerError> for Prompter {}

#[async_trait]
impl asimov_patterns::Execute<String, RunnerError> for Prompter {
    async fn execute(&mut self) -> ProviderResult {
        let mut process = self.runner.spawn().await?;

        let prompt = self.prompt.clone();
        let mut stdin = process.stdin.take().expect("should capture stdin");
        tokio::spawn(async move {
            use tokio::io::AsyncWriteExt;
            stdin
                .write_all(prompt.to_string().as_bytes())
                .await
                .expect("should write to stdin");
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
        let mut runner = Prompter::new(
            "cat",
            Prompt::builder()
                .messages(vec![PromptMessage(
                    PromptRole::User,
                    "Hello, world!".into(),
                )])
                .build(),
            PrompterOptions::default(),
        );
        let result = runner.execute().await;
        assert!(result.is_ok());
        assert_eq!(result.unwrap(), String::from("user: Hello, world!\n"));
    }
}

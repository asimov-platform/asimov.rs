// This is free and unencumbered software released into the public domain.

use crate::{Executor, ExecutorError, Input, TextOutput};
use async_trait::async_trait;
use derive_more::Debug;
use std::{
    ffi::OsStr,
    io::{Cursor, Read},
    process::Stdio,
};
use tokio::io::{AsyncRead, AsyncWrite};

pub use asimov_patterns::PrompterOptions;
pub use asimov_prompt::{Prompt, PromptMessage, PromptRole};

pub type PrompterResult = std::result::Result<String, ExecutorError>;

/// See: https://asimov-specs.github.io/program-patterns/#prompter
#[allow(unused)]
#[derive(Debug)]
pub struct Prompter {
    executor: Executor,
    options: PrompterOptions,
    input: Prompt,
    output: TextOutput,
}

impl Prompter {
    pub fn new(
        program: impl AsRef<OsStr>,
        input: Prompt,
        output: TextOutput,
        options: PrompterOptions,
    ) -> Self {
        let mut executor = Executor::new(program);

        executor
            .command()
            .args(if let Some(ref input) = options.input {
                vec![format!("--input={}", input)]
            } else {
                vec![]
            })
            .args(if let Some(ref output) = options.output {
                vec![format!("--output={}", output)]
            } else {
                vec![]
            })
            .args(if let Some(ref model) = options.model {
                vec![format!("--model={}", model)]
            } else {
                vec![]
            })
            .args(&options.other)
            .stdin(Stdio::piped())
            .stdout(Stdio::piped())
            .stderr(Stdio::piped());

        Self {
            executor,
            options,
            input,
            output,
        }
    }
}

impl asimov_patterns::Prompter<String, ExecutorError> for Prompter {}

#[async_trait]
impl asimov_patterns::Execute<String, ExecutorError> for Prompter {
    async fn execute(&mut self) -> PrompterResult {
        let mut process = self.executor.spawn().await?;

        let prompt = self.input.clone();
        let mut stdin = process.stdin.take().expect("should capture stdin");
        tokio::spawn(async move {
            use tokio::io::AsyncWriteExt;
            stdin
                .write_all(prompt.to_string().as_bytes())
                .await
                .expect("should write to stdin");
        });

        let mut stdout = self.executor.wait(process).await?;
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
        let mut prompter = Prompter::new(
            "cat",
            Prompt::builder()
                .messages(vec![PromptMessage(
                    PromptRole::User,
                    "Hello, world!".into(),
                )])
                .build(),
            TextOutput::Ignored,
            PrompterOptions::default(),
        );
        let result = prompter.execute().await;
        assert!(result.is_ok());
        assert_eq!(result.unwrap(), String::from("user: Hello, world!\n"));
    }
}

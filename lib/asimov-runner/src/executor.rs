// This is free and unencumbered software released into the public domain.

use crate::{Command, ExecutorError, ExecutorResult};
use std::{
    ffi::OsStr,
    io::{Cursor, ErrorKind},
    os::fd::AsFd,
    process::Stdio,
};
use tokio::{io::AsyncReadExt, process::Child};

#[derive(Debug)]
pub struct Executor(Command);

impl Executor {
    pub fn new(program: impl AsRef<OsStr>) -> Self {
        let mut command = Command::new(program);
        command.env("NO_COLOR", "1"); // See: https://no-color.org
        command.stdin(Stdio::null());
        command.stdout(Stdio::null());
        command.stderr(Stdio::null());
        command.kill_on_drop(true);
        Self(command)
    }

    pub fn command(&mut self) -> &mut Command {
        &mut self.0
    }

    pub fn ignore_stdin(&mut self) {
        self.0.stdin(Stdio::null());
    }

    pub fn ignore_stdout(&mut self) {
        self.0.stdout(Stdio::null());
    }

    pub fn ignore_stderr(&mut self) {
        self.0.stderr(Stdio::null());
    }

    pub fn capture_stdout(&mut self) {
        self.0.stdout(Stdio::piped());
    }

    pub fn capture_stderr(&mut self) {
        self.0.stderr(Stdio::piped());
    }

    pub async fn spawn(&mut self) -> Result<Child, ExecutorError> {
        match self.0.spawn() {
            Ok(process) => Ok(process),
            Err(err) if err.kind() == ErrorKind::NotFound => {
                let program = self.0.as_std().get_program().to_owned();
                return Err(ExecutorError::MissingProgram(program));
            },
            Err(err) => return Err(ExecutorError::SpawnFailure(err)),
        }
    }

    pub async fn wait(&mut self, process: Child) -> ExecutorResult {
        let output = process.wait_with_output().await?;

        #[cfg(feature = "tracing")]
        tracing::trace!("The command exited with: {}", output.status);

        if !output.status.success() {
            return Err(output.into());
        }

        Ok(Cursor::new(output.stdout))
    }

    pub async fn execute(&mut self) -> ExecutorResult {
        let process = self.spawn().await?;
        self.wait(process).await
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_success() {
        let mut runner = Executor::new("curl");
        runner.command().arg("https://google.com");
        let result = runner.execute().await;
        assert!(result.is_ok());
    }

    #[tokio::test]
    async fn test_missing_program() {
        let mut runner = Executor::new("this-command-does-not-exist");
        let result = runner.execute().await;
        assert!(matches!(result, Err(ExecutorError::MissingProgram(_))));
    }

    #[cfg(unix)]
    #[tokio::test]
    async fn test_spawn_failure() {
        let mut runner = Executor::new("/dev/null");
        let result = runner.execute().await;
        assert!(matches!(result, Err(ExecutorError::SpawnFailure(_))));
    }

    #[tokio::test]
    async fn test_unexpected_failure() {
        let mut runner = Executor::new("curl");
        let result = runner.execute().await;
        assert!(matches!(
            result,
            Err(ExecutorError::UnexpectedFailure(_, _))
        ));
    }
}

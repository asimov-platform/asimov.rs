// This is free and unencumbered software released into the public domain.

use crate::{Command, SysexitsError};
use std::{
    ffi::{OsStr, OsString},
    io::{Cursor, ErrorKind},
    process::{ExitStatus, Output, Stdio},
};

#[derive(Debug)]
pub struct Runner(Command);

impl Runner {
    pub fn new(program: impl AsRef<OsStr>) -> Self {
        let mut command = Command::new(program);
        command.env("NO_COLOR", "1");
        command.stdin(Stdio::null());
        command.stdout(Stdio::null());
        command.stderr(Stdio::null());
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

    pub async fn execute(&mut self) -> RunnerResult {
        let process = match self.0.spawn() {
            Ok(process) => process,
            Err(err) if err.kind() == ErrorKind::NotFound => {
                let program = self.0.as_std().get_program().to_owned();
                return Err(RunnerError::MissingProgram(program));
            }
            Err(err) => return Err(RunnerError::SpawnFailure(err)),
        };

        let output = process.wait_with_output().await?;
        tracing::trace!("The command exited with: {}", output.status);

        if !output.status.success() {
            return Err(output.into());
        }

        Ok(Cursor::new(output.stdout))
    }
}

pub type RunnerResult = std::result::Result<Cursor<Vec<u8>>, RunnerError>;

#[derive(Debug)]
pub enum RunnerError {
    MissingProgram(OsString),
    SpawnFailure(std::io::Error),
    Failure(SysexitsError, Option<String>),
    UnexpectedFailure(Option<i32>, Option<String>),
    UnexpectedOther(std::io::Error),
}

impl From<Output> for RunnerError {
    fn from(output: Output) -> Self {
        let stderr = String::from_utf8(output.stderr).ok();
        match SysexitsError::try_from(output.status) {
            Ok(error) => Self::Failure(error, stderr),
            Err(code) => Self::UnexpectedFailure(code, stderr),
        }
    }
}

impl From<ExitStatus> for RunnerError {
    fn from(status: ExitStatus) -> Self {
        match SysexitsError::try_from(status) {
            Ok(error) => Self::Failure(error, None),
            Err(code) => Self::UnexpectedFailure(code, None),
        }
    }
}

impl From<std::io::Error> for RunnerError {
    fn from(error: std::io::Error) -> Self {
        Self::UnexpectedOther(error)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_success() {
        let mut runner = Runner::new("curl");
        runner.command().arg("http://neverssl.com");
        let result = runner.execute().await;
        assert!(result.is_ok());
    }

    #[tokio::test]
    async fn test_missing_program() {
        let mut runner = Runner::new("this-command-does-not-exist");
        let result = runner.execute().await;
        assert!(matches!(result, Err(RunnerError::MissingProgram(_))));
    }

    #[tokio::test]
    async fn test_spawn_failure() {
        let mut runner = Runner::new("/dev/null");
        let result = runner.execute().await;
        assert!(matches!(result, Err(RunnerError::SpawnFailure(_))));
    }

    #[tokio::test]
    async fn test_unexpected_failure() {
        let mut runner = Runner::new("curl");
        let result = runner.execute().await;
        assert!(matches!(result, Err(RunnerError::UnexpectedFailure(_, _))));
    }
}

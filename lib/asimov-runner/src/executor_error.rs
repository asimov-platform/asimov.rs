// This is free and unencumbered software released into the public domain.

use crate::SysexitsError;
use alloc::{string::String, vec::Vec};
use core::fmt;
use std::{ffi::OsString, io::Cursor};

pub type ExecutorResult = std::result::Result<Cursor<Vec<u8>>, ExecutorError>;

#[derive(Debug)]
pub enum ExecutorError {
    MissingProgram(OsString),
    SpawnFailure(std::io::Error),
    Failure(SysexitsError, Option<String>),
    UnexpectedFailure(Option<i32>, Option<String>),
    UnexpectedOther(std::io::Error),
}

impl core::error::Error for ExecutorError {}

impl fmt::Display for ExecutorError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::MissingProgram(program) => {
                write!(f, "Missing program: {}", program.to_string_lossy())
            },
            Self::SpawnFailure(err) => write!(f, "Failed to spawn process: {}", err),
            Self::Failure(error, stderr) => {
                write!(
                    f,
                    "Command failed with exit code {}",
                    error.code().unwrap_or(-1),
                )?;
                if let Some(stderr) = stderr {
                    write!(f, "\n{}", stderr)?;
                }
                Ok(())
            },
            Self::UnexpectedFailure(code, stderr) => {
                write!(
                    f,
                    "Command failed with unexpected exit code: {}",
                    code.unwrap_or(-1)
                )?;
                if let Some(stderr) = stderr {
                    write!(f, "\n{}", stderr)?;
                }
                Ok(())
            },
            Self::UnexpectedOther(err) => write!(f, "Unexpected error: {}", err),
        }
    }
}

#[cfg(feature = "std")]
impl From<std::io::Error> for ExecutorError {
    fn from(error: std::io::Error) -> Self {
        Self::UnexpectedOther(error)
    }
}

#[cfg(feature = "std")]
impl From<std::process::ExitStatus> for ExecutorError {
    fn from(status: std::process::ExitStatus) -> Self {
        match SysexitsError::try_from(status) {
            Ok(error) => Self::Failure(error, None),
            Err(code) => Self::UnexpectedFailure(code, None),
        }
    }
}

#[cfg(feature = "std")]
impl From<std::process::Output> for ExecutorError {
    fn from(output: std::process::Output) -> Self {
        let stderr = String::from_utf8(output.stderr).ok();
        match SysexitsError::try_from(output.status) {
            Ok(error) => Self::Failure(error, stderr),
            Err(code) => Self::UnexpectedFailure(code, stderr),
        }
    }
}

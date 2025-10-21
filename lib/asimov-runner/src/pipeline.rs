// This is free and unencumbered software released into the public domain.

use crate::{Execute, ExecutorError};
use std::io::{Cursor, Read};
use std::process::{ChildStdin, ChildStdout, Stdio};

mod step;
pub use step::PipelineStep;

pub struct Pipeline {
    pub steps: Vec<PipelineStep>,
}

impl Pipeline {
    pub fn execute(self) -> Result<Option<Cursor<Vec<u8>>>, ExecutorError> {
        let mut children = Vec::with_capacity(self.steps.len());
        let mut last_stdout: Option<ChildStdout> = None;

        for step in self.steps.into_iter() {
            let mut cmd = step.to_command();

            if let Some(stdout) = last_stdout.take() {
                cmd.stdin(std::process::Stdio::from(stdout));
                cmd.stdout(Stdio::piped());
            } else {
                cmd.stdin(Stdio::null());
                cmd.stdout(Stdio::piped());
            }

            let mut child = cmd.spawn()?;
            last_stdout = child.stdout.take();

            children.push(child);
        }

        for child in &mut children {
            child.wait()?;
        }

        if let Some(mut stdout) = last_stdout.take() {
            let mut output = Vec::new();
            stdout.read_to_end(&mut output)?;
            return Ok(Some(Cursor::new(output)));
        }

        Ok(None)
    }
}

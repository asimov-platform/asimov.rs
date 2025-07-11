// This is free and unencumbered software released into the public domain.

use super::FlowExecutionState;
use crate::{
    Named,
    prelude::{Cow, String, fmt::Debug},
};
use asimov_sys::{AsiFlowExecution, AsiFlowExecutionState};

#[derive(Debug)]
pub struct FlowExecution {
    pub(crate) inner: AsiFlowExecution,
}

impl FlowExecution {
    pub fn new(name: &str, timestamp: u64, pid: u64, state: AsiFlowExecutionState) -> Self {
        Self {
            inner: AsiFlowExecution::new(name, timestamp, pid, state),
        }
    }

    pub fn timestamp(&self) -> u64 {
        self.inner.timestamp
    }

    pub fn pid(&self) -> Option<u64> {
        match self.inner.pid {
            0 => None,
            pid => Some(pid),
        }
    }

    pub fn state(&self) -> FlowExecutionState {
        self.inner.state.into()
    }
}

impl From<AsiFlowExecution> for FlowExecution {
    fn from(inner: AsiFlowExecution) -> Self {
        Self { inner }
    }
}

impl Named for FlowExecution {
    fn name(&self) -> Cow<str> {
        self.inner.name_lossy()
    }
}

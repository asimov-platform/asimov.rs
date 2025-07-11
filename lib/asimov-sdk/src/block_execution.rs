// This is free and unencumbered software released into the public domain.

use super::flow::FlowExecutionState;
use crate::{
    Named,
    prelude::{Cow, String, fmt::Debug},
};
use asimov_sys::{AsiBlockExecution, AsiFlowExecutionState};

#[derive(Debug)]
pub struct BlockExecution {
    pub(crate) inner: AsiBlockExecution,
}

impl BlockExecution {
    pub fn new(name: &str, timestamp: u64, pid: u64, state: AsiFlowExecutionState) -> Self {
        Self {
            inner: AsiBlockExecution::new(name, timestamp, pid, state),
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

impl From<AsiBlockExecution> for BlockExecution {
    fn from(inner: AsiBlockExecution) -> Self {
        Self { inner }
    }
}

impl Named for BlockExecution {
    fn name(&self) -> Cow<str> {
        self.inner.name_lossy()
    }
}

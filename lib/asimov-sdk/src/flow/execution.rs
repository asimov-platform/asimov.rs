// This is free and unencumbered software released into the public domain.

use crate::{
    prelude::{fmt::Debug, Cow, String},
    Named,
};
use asimov_sys::{AsiFlowExecution, AsiFlowExecutionState};

#[derive(Debug)]
pub struct FlowExecution {
    pub(crate) inner: AsiFlowExecution,
}

impl FlowExecution {
    pub fn new(name: &str, pid: u64, state: AsiFlowExecutionState) -> Self {
        Self {
            inner: AsiFlowExecution::new(name, pid, state),
        }
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

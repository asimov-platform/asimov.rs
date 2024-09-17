// This is free and unencumbered software released into the public domain.

use crate::{
    prelude::{fmt::Debug, Cow, String},
    Named,
};
use asimov_sys::{AsiFlowExecution, AsiFlowExecutionState};

#[stability::unstable]
pub trait FlowExecution: Named + Debug {}

#[derive(Debug)]
pub(crate) struct LocalFlowExecution {
    inner: AsiFlowExecution,
}

impl LocalFlowExecution {
    #[allow(unused)]
    pub fn new(name: &str, pid: u64, state: AsiFlowExecutionState) -> Self {
        Self {
            inner: AsiFlowExecution::new(name, pid, state),
        }
    }
}

impl From<AsiFlowExecution> for LocalFlowExecution {
    fn from(inner: AsiFlowExecution) -> Self {
        Self { inner }
    }
}

impl Named for LocalFlowExecution {
    fn name(&self) -> Cow<str> {
        self.inner.name_lossy()
    }
}

impl FlowExecution for LocalFlowExecution {}

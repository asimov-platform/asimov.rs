// This is free and unencumbered software released into the public domain.

use asimov_sys::AsiFlowExecutionState;
use core::fmt;

#[derive(Debug, Default)]
pub enum FlowExecutionState {
    #[default]
    Unknown,
    Started,
    Completed,
    Failed,
}

impl FlowExecutionState {
    fn as_str(&self) -> &'static str {
        use FlowExecutionState::*;
        match self {
            Unknown => "unknown",
            Started => "started",
            Completed => "completed",
            Failed => "failed",
        }
    }
}

impl From<AsiFlowExecutionState> for FlowExecutionState {
    fn from(state: AsiFlowExecutionState) -> Self {
        use AsiFlowExecutionState::*;
        match state {
            ASI_FLOW_EXECUTION_STATE_UNKNOWN => FlowExecutionState::Unknown,
            ASI_FLOW_EXECUTION_STATE_STARTED => FlowExecutionState::Started,
            ASI_FLOW_EXECUTION_STATE_COMPLETED => FlowExecutionState::Completed,
            ASI_FLOW_EXECUTION_STATE_FAILED => FlowExecutionState::Failed,
        }
    }
}

impl Into<AsiFlowExecutionState> for FlowExecutionState {
    fn into(self) -> AsiFlowExecutionState {
        use FlowExecutionState::*;
        match self {
            Unknown => AsiFlowExecutionState::ASI_FLOW_EXECUTION_STATE_UNKNOWN,
            Started => AsiFlowExecutionState::ASI_FLOW_EXECUTION_STATE_STARTED,
            Completed => AsiFlowExecutionState::ASI_FLOW_EXECUTION_STATE_COMPLETED,
            Failed => AsiFlowExecutionState::ASI_FLOW_EXECUTION_STATE_FAILED,
        }
    }
}

impl fmt::Display for FlowExecutionState {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.as_str())
    }
}

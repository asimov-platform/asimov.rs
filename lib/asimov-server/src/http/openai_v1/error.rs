// This is free and unencumbered software released into the public domain.

#![allow(unused_imports)]

use asimov_runner::RunnerError;
use axum::{
    Json,
    http::StatusCode,
    response::{IntoResponse, Response},
};
use openai::schemas::Error;

#[derive(Debug, thiserror::Error)]
pub enum CompletionError {
    #[error("no model specified")]
    EmptyModel,

    #[error("no prompt specified")]
    EmptyPrompt,

    #[error("feature not implemented: {0}")]
    UnimplementedFeature(String),

    #[error("execution failed: {0}")]
    FailedExecute(#[from] RunnerError),
}

impl IntoResponse for CompletionError {
    fn into_response(self) -> Response {
        (
            StatusCode::BAD_REQUEST,
            Json(Error {
                message: self.to_string(),
                ..Default::default()
            }),
        )
            .into_response()
    }
}

// This is free and unencumbered software released into the public domain.

#![allow(unused_imports)]

use axum::{
    Json, Router, extract,
    http::StatusCode,
    response::{IntoResponse, Response},
    routing::post,
};
use jiff::Timestamp;
use openai::components::{
    CompletionUsage, CreateCompletionRequest, CreateCompletionResponse, Error,
};

/// See: https://platform.openai.com/docs/api-reference/completions
pub fn routes() -> Router {
    Router::new().route("/", post(create))
}

/// See: https://platform.openai.com/docs/api-reference/completions/create
#[axum::debug_handler]
async fn create(
    extract::Json(request): extract::Json<CreateCompletionRequest>,
) -> Result<Json<CreateCompletionResponse>, CreateCompletionError> {
    if request.model.is_empty() {
        return Err(CreateCompletionError::EmptyModel);
    }
    if request.prompt.is_none() {
        return Err(CreateCompletionError::EmptyPrompt);
    }

    // See: https://platform.openai.com/docs/api-reference/completions/object
    Ok(Json(CreateCompletionResponse {
        id: String::from("cmpl-uqkvlQyYK7bGYrRHQ0eXlWi7"), // TODO
        choices: vec![],
        created: Timestamp::now().as_second(),
        model: String::new(),
        system_fingerprint: String::new(),
        object: "text_completion".to_string(),
        usage: CompletionUsage {
            completion_tokens: 0,
            prompt_tokens: 0,
            total_tokens: 0,
            completion_tokens_details: Default::default(),
            prompt_tokens_details: Default::default(),
        },
    })) // TODO
}

#[derive(Debug, thiserror::Error)]
enum CreateCompletionError {
    #[error("no model specified")]
    EmptyModel,
    #[error("no prompt specified")]
    EmptyPrompt,
}

impl IntoResponse for CreateCompletionError {
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

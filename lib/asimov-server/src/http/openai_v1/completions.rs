// This is free and unencumbered software released into the public domain.

#![allow(unused_imports)]

use asimov_runner::{Execute, Provider, ProviderOptions, RunnerError};
use axum::{
    Json, Router, extract,
    http::StatusCode,
    response::{IntoResponse, Response},
    routing::post,
};
use jiff::Timestamp;
use openai::components::{
    CompletionUsage, CreateCompletionRequest, CreateCompletionRequest_Prompt,
    CreateCompletionResponse, CreateCompletionResponse_Choices, Error,
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
    use CreateCompletionRequest_Prompt as Prompt;

    if request.model.is_empty() {
        return Err(CreateCompletionError::EmptyModel);
    }
    if request.prompt.is_none() {
        return Err(CreateCompletionError::EmptyPrompt);
    }

    let mut provider = Provider::new(
        "asimov-default-provider",
        ProviderOptions {
            prompt: match request.prompt.unwrap() {
                Prompt::String(prompt) => prompt.into(),
                Prompt::ArrayOfStrings(prompts) => prompts.join("").into(),
                Prompt::ArrayOfIntegers(_) => {
                    return Err(CreateCompletionError::UnimplementedFeature(
                        "prompt from an array of tokens".into(),
                    ));
                }
                Prompt::Array(_) => {
                    return Err(CreateCompletionError::UnimplementedFeature(
                        "prompt from an array of token arrays".into(),
                    ));
                }
            },
        },
    );

    let provider_output = provider
        .execute()
        .await
        .map_err(|e| CreateCompletionError::FailedExecute(e))?;

    // See: https://platform.openai.com/docs/api-reference/completions/object
    Ok(Json(CreateCompletionResponse {
        id: super::util::generate_openai_id("cmpl"),
        object: "text_completion".into(),
        created: Timestamp::now().as_second(),
        model: request.model,
        choices: vec![CreateCompletionResponse_Choices {
            text: provider_output,
            index: 0,
            logprobs: None,
            finish_reason: "stop".into(),
        }],
        system_fingerprint: String::new(), // TODO: module name
        usage: CompletionUsage {
            prompt_tokens: 0,     // TODO
            completion_tokens: 0, // TODO
            total_tokens: 0,      // TODO
            prompt_tokens_details: Default::default(),
            completion_tokens_details: Default::default(),
        },
    })) // TODO
}

#[derive(Debug, thiserror::Error)]
enum CreateCompletionError {
    #[error("no model specified")]
    EmptyModel,

    #[error("no prompt specified")]
    EmptyPrompt,

    #[error("feature not implemented: {0}")]
    UnimplementedFeature(String),

    #[error("execution failed: {0}")]
    FailedExecute(#[from] RunnerError),
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

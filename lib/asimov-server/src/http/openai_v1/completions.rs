// This is free and unencumbered software released into the public domain.

#![allow(unused_imports)]

use std::sync::{Arc, RwLock};

use crate::persistence::{self, PersistentState};

use super::error::CompletionError;
use asimov_runner::{Execute, Prompt, Provider, ProviderOptions};
use axum::{Json, Router, extract, routing::post};
use jiff::Timestamp;
use openai::schemas::{
    CompletionUsage, CreateCompletionRequest, CreateCompletionResponse,
    CreateCompletionResponse_Choices,
};

/// See: https://platform.openai.com/docs/api-reference/completions
pub fn routes() -> Router {
    Router::new()
        .route("/", post(create))
        .with_state(persistence::get_ref())
}

/// See: https://platform.openai.com/docs/api-reference/completions/create
#[axum::debug_handler]
async fn create(
    extract::State(state): extract::State<Arc<RwLock<PersistentState>>>,
    extract::Json(request): extract::Json<CreateCompletionRequest>,
) -> Result<Json<CreateCompletionResponse>, CompletionError> {
    if request.model.is_empty() {
        return Err(CompletionError::EmptyModel);
    }
    if request.prompt.is_none() {
        return Err(CompletionError::EmptyPrompt);
    }

    let provider_name = state.read().unwrap().provider.clone();
    let mut provider = Provider::new(
        provider_name,
        ProviderOptions {
            prompt: Prompt::try_from(request.prompt.unwrap()).map_err(|_| {
                CompletionError::UnimplementedFeature("prompt from an array of tokens".into())
            })?,
            ..Default::default()
        },
    );

    let provider_output = provider
        .execute()
        .await
        .map_err(|e| CompletionError::FailedExecute(e))?;

    // See: https://platform.openai.com/docs/api-reference/completions/object
    Ok(Json(CreateCompletionResponse {
        id: super::util::generate_openai_id("cmpl"),
        object: "text_completion".into(),
        created: Timestamp::now().as_second(),
        model: request.model,
        choices: vec![CreateCompletionResponse_Choices {
            index: 0,
            finish_reason: "stop".into(),
            logprobs: None,
            text: provider_output,
        }],
        system_fingerprint: None,
        usage: Some(CompletionUsage {
            prompt_tokens: 0,     // TODO
            completion_tokens: 0, // TODO
            total_tokens: 0,      // TODO
            prompt_tokens_details: Default::default(),
            completion_tokens_details: Default::default(),
        }),
    })) // TODO
}

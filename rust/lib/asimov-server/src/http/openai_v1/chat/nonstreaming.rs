// This is free and unencumbered software released into the public domain.

use crate::{
    http::openai_v1::{error::CompletionError, util::generate_openai_id},
    persistence::PersistentState,
};
use asimov_prompt::{Prompt, PromptMessage, PromptRole};
use asimov_runner::{Execute, Prompter, PrompterOptions, TextOutput};
use axum::Json;
use jiff::Timestamp;
use openai::schemas::{
    ChatCompletionRequestMessage, ChatCompletionResponseMessage,
    CreateChatCompletionRequest_Variant2 as CreateChatCompletionRequest,
    CreateChatCompletionResponse, CreateChatCompletionResponse_Choices,
};
use std::sync::{Arc, RwLock};

/// See: https://platform.openai.com/docs/api-reference/chat/create
pub async fn create(
    state: Arc<RwLock<PersistentState>>,
    request: CreateChatCompletionRequest,
) -> Result<Json<CreateChatCompletionResponse>, CompletionError> {
    let id = generate_openai_id("chatcmpl");
    let model = request.model;

    let mut prompt_messages: Vec<PromptMessage> = Vec::new();
    for request_message in &request.messages {
        let _: &ChatCompletionRequestMessage = request_message;
        prompt_messages.push(
            PromptMessage::try_from(request_message)
                .map_err(|_| CompletionError::UnimplementedFeature("complex prompt".into()))?,
        );
    }
    let prompt = Prompt::from(prompt_messages);

    let provider_name = state.read().unwrap().provider.clone();
    let mut provider = Prompter::new(
        provider_name,
        prompt,
        TextOutput::Captured,
        PrompterOptions::default(),
    );
    let provider_output = provider
        .execute()
        .await
        .map_err(|error| CompletionError::FailedExecute(error))?;

    Ok(Json(CreateChatCompletionResponse {
        id,
        object: "chat.completion".to_string(),
        created: Timestamp::now().as_second(),
        model,
        choices: vec![CreateChatCompletionResponse_Choices {
            index: 0,
            finish_reason: "stop".into(),
            message: ChatCompletionResponseMessage {
                role: PromptRole::Assistant.into(),
                content: Some(provider_output),
                ..Default::default()
            },
            logprobs: None,
        }],
        ..Default::default()
    }))
}

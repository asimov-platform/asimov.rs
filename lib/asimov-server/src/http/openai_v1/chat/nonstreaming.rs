// This is free and unencumbered software released into the public domain.

use crate::http::openai_v1::{error::CompletionError, util::generate_openai_id};
use asimov_prompt::{Prompt, PromptMessage, PromptRole};
use asimov_runner::{Execute, Provider, ProviderOptions};
use axum::Json;
use jiff::Timestamp;
use openai::schemas::{
    ChatCompletionRequestMessage, ChatCompletionResponseMessage,
    CreateChatCompletionRequest_Variant2 as CreateChatCompletionRequest,
    CreateChatCompletionResponse, CreateChatCompletionResponse_Choices,
};

/// See: https://platform.openai.com/docs/api-reference/chat/create
pub async fn create(
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

    let mut provider = Provider::new("asimov-default-provider", ProviderOptions { prompt }); // TODO
    let provider_output = provider.execute().await?;

    Ok(Json(CreateChatCompletionResponse {
        id: id.clone(),
        object: "chat.completion.chunk".to_string(),
        created: Timestamp::now().as_second(),
        model: model.clone(),
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

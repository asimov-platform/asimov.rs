// This is free and unencumbered software released into the public domain.

use crate::{
    http::openai_v1::{error::CompletionError, util::generate_openai_id},
    persistence::PersistentState,
};
use asimov_prompt::{Prompt, PromptMessage, PromptRole};
use asimov_runner::{Execute, Prompter, PrompterOptions};
use async_stream::try_stream;
use axum::response::sse::{Event, Sse};
use futures::stream::Stream;
use jiff::Timestamp;
use openai::schemas::{
    ChatCompletionRequestMessage, ChatCompletionStreamResponseDelta,
    CreateChatCompletionRequest_Variant2 as CreateChatCompletionRequest,
    CreateChatCompletionStreamResponse, CreateChatCompletionStreamResponse_Choices,
};
use std::sync::{Arc, RwLock};

/// See: https://platform.openai.com/docs/api-reference/chat/create
pub async fn create(
    state: Arc<RwLock<PersistentState>>,
    request: CreateChatCompletionRequest,
) -> Result<Sse<impl Stream<Item = Result<Event, axum::Error>>>, CompletionError> {
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
    let mut provider = Prompter::new(provider_name, prompt, PrompterOptions::default());
    let provider_output = provider
        .execute()
        .await
        .map_err(|error| CompletionError::FailedExecute(error))?;

    Ok(Sse::new(try_stream!({
        yield Event::default().json_data(CreateChatCompletionStreamResponse {
            id: id.clone(),
            object: "chat.completion.chunk".to_string(),
            created: Timestamp::now().as_second(),
            model: model.clone(),
            choices: vec![CreateChatCompletionStreamResponse_Choices {
                index: 0,
                finish_reason: None,
                delta: ChatCompletionStreamResponseDelta {
                    role: Some(PromptRole::Assistant.into()),
                    content: Some(provider_output),
                    ..Default::default()
                },
                logprobs: None,
            }],
            ..Default::default()
        })?;

        yield Event::default().json_data(CreateChatCompletionStreamResponse {
            id: id.clone(),
            object: "chat.completion.chunk".to_string(),
            created: Timestamp::now().as_second(),
            model: model.clone(),
            choices: vec![CreateChatCompletionStreamResponse_Choices {
                index: 0,
                finish_reason: Some("stop".into()),
                delta: ChatCompletionStreamResponseDelta {
                    role: Some(PromptRole::Assistant.into()),
                    content: Some(String::new()),
                    ..Default::default()
                },
                logprobs: None,
            }],
            ..Default::default()
        })?;

        yield Event::default().data("[DONE]");
    })))
}

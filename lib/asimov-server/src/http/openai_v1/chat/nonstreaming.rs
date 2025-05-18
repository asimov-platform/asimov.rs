// This is free and unencumbered software released into the public domain.

use axum::Json;
use openai::schemas::{
    CreateChatCompletionRequest_Variant2 as CreateChatCompletionRequest,
    CreateChatCompletionResponse,
};

/// See: https://platform.openai.com/docs/api-reference/chat/create
pub async fn create(_request: CreateChatCompletionRequest) -> Json<CreateChatCompletionResponse> {
    Json(super::dummy_response()) // TODO
}

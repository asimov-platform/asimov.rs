// This is free and unencumbered software released into the public domain.

use axum::Json;
use openai::schemas::{
    CreateChatCompletionRequest_Variant2 as CreateChatCompletionRequest,
    CreateChatCompletionStreamResponse,
};

/// See: https://platform.openai.com/docs/api-reference/chat/create
pub async fn create(
    _request: CreateChatCompletionRequest,
) -> Json<CreateChatCompletionStreamResponse> {
    let _response: CreateChatCompletionStreamResponse = todo!(); // TODO
    #[allow(unreachable_code)]
    Json(_response)
}

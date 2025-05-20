// This is free and unencumbered software released into the public domain.

#![allow(unused_imports)]

use axum::{Json, Router, extract, routing::post};
use openai::schemas::{
    CreateSpeechRequest, CreateTranscriptionRequest, CreateTranscriptionResponseJson,
    CreateTranslationRequest, CreateTranslationResponseJson,
};

/// See: https://platform.openai.com/docs/api-reference/audio
pub fn routes() -> Router {
    Router::new()
        .route("/speech", post(create_speech))
        .route("/transcriptions", post(create_transcription))
        .route("/translations", post(create_translation))
}

/// See: https://platform.openai.com/docs/api-reference/audio/createSpeech
#[axum::debug_handler]
async fn create_speech(extract::Json(_): extract::Json<CreateSpeechRequest>) -> Vec<u8> {
    Vec::new() // TODO
}

/// See: https://platform.openai.com/docs/api-reference/audio/createTranscription
#[axum::debug_handler]
async fn create_transcription(
    extract::Json(_): extract::Json<CreateTranscriptionRequest>,
) -> Json<CreateTranscriptionResponseJson> {
    Json(CreateTranscriptionResponseJson {
        text: String::new(), // TODO
        logprobs: None,
    })
}

/// See: https://platform.openai.com/docs/api-reference/audio/createTranslation
#[axum::debug_handler]
async fn create_translation(
    extract::Json(_): extract::Json<CreateTranslationRequest>,
) -> Json<CreateTranslationResponseJson> {
    Json(CreateTranslationResponseJson::default()) // TODO
}

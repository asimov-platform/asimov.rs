// This is free and unencumbered software released into the public domain.

#![allow(unused_imports)]

use axum::{Json, Router, extract, routing::post};
use openai::components::{
    CreateSpeechRequest, CreateTranscriptionRequest, CreateTranslationRequest,
    CreateTranslationResponseJson, CreateTranslationResponseVerboseJson,
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
async fn create_speech(extract::Json(_): extract::Json<CreateSpeechRequest>) -> Json<bool> {
    Json(false) // TODO
}

/// See: https://platform.openai.com/docs/api-reference/audio/createTranscription
#[axum::debug_handler]
async fn create_transcription(
    extract::Json(_): extract::Json<CreateTranscriptionRequest>,
) -> Json<bool> {
    Json(false) // TODO
}

/// See: https://platform.openai.com/docs/api-reference/audio/createTranslation
#[axum::debug_handler]
async fn create_translation(
    extract::Json(_): extract::Json<CreateTranslationRequest>,
) -> Json<bool> {
    Json(false) // TODO
}

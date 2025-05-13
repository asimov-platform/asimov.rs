// This is free and unencumbered software released into the public domain.

use axum::{Json, Router, routing::post};

/// See: https://platform.openai.com/docs/api-reference/audio
pub fn routes() -> Router {
    Router::new()
        .route("/speech", post(create_speech))
        .route("/transcriptions", post(create_transcription))
        .route("/translations", post(create_translation))
}

/// See: https://platform.openai.com/docs/api-reference/audio/createSpeech
#[axum::debug_handler]
async fn create_speech() -> Json<bool> {
    Json(false) // TODO
}

/// See: https://platform.openai.com/docs/api-reference/audio/createTranscription
#[axum::debug_handler]
async fn create_transcription() -> Json<bool> {
    Json(false) // TODO
}

/// See: https://platform.openai.com/docs/api-reference/audio/createTranslation
#[axum::debug_handler]
async fn create_translation() -> Json<bool> {
    Json(false) // TODO
}

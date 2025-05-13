// This is free and unencumbered software released into the public domain.

pub mod audio;
pub mod chat;
pub mod completions;
pub mod embeddings;
pub mod images;
pub mod models;
pub mod realtime;
pub mod responses;

use axum::Router;

pub fn routes() -> Router {
    Router::new()
        .nest("/audio", audio::routes())
        .nest("/chat", chat::routes())
        .nest("/completions", completions::routes())
        .nest("/embeddings", embeddings::routes())
        .nest("/images", images::routes())
        .nest("/models", models::routes())
        .nest("/realtime", realtime::routes())
        .nest("/responses", responses::routes())
}

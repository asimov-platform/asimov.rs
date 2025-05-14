// This is free and unencumbered software released into the public domain.

#![allow(unused_imports)]

use axum::{Json, Router, extract, routing::post};
use jiff::Timestamp;
use openai::components::{
    CreateImageEditRequest, CreateImageRequest, CreateImageVariationRequest, ImagesResponse,
    ImagesResponse_Usage,
};

/// See: https://platform.openai.com/docs/api-reference/images
pub fn routes() -> Router {
    Router::new()
        .route("/generations", post(create))
        .route("/edits", post(create_edit))
        .route("/variations", post(create_variation))
}

/// See: https://platform.openai.com/docs/api-reference/images/create
#[axum::debug_handler]
async fn create(extract::Json(_): extract::Json<CreateImageRequest>) -> Json<ImagesResponse> {
    Json(ImagesResponse {
        data: vec![], // TODO
        created: Timestamp::now().as_second(),
        usage: ImagesResponse_Usage {
            total_tokens: 0,
            input_tokens: 0,
            output_tokens: 0,
            input_tokens_details: Default::default(),
        },
    })
}

/// See: https://platform.openai.com/docs/api-reference/images/createEdit
#[axum::debug_handler]
async fn create_edit(
    extract::Json(_): extract::Json<CreateImageEditRequest>,
) -> Json<ImagesResponse> {
    Json(ImagesResponse {
        data: vec![], // TODO
        created: Timestamp::now().as_second(),
        usage: ImagesResponse_Usage {
            total_tokens: 0,
            input_tokens: 0,
            output_tokens: 0,
            input_tokens_details: Default::default(),
        },
    })
}

/// See: https://platform.openai.com/docs/api-reference/images/createVariation
#[axum::debug_handler]
async fn create_variation(
    extract::Json(_): extract::Json<CreateImageVariationRequest>,
) -> Json<ImagesResponse> {
    Json(ImagesResponse {
        data: vec![], // TODO
        created: Timestamp::now().as_second(),
        usage: ImagesResponse_Usage {
            total_tokens: 0,
            input_tokens: 0,
            output_tokens: 0,
            input_tokens_details: Default::default(),
        },
    })
}

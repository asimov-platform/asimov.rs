// This is free and unencumbered software released into the public domain.

#![allow(unused_imports)]

use axum::{
    Json, Router, extract,
    routing::{delete, get, post},
};
use openai::components::{CreateResponse, Error, Response, ResponseItemList, ResponseStreamEvent};

/// See: https://platform.openai.com/docs/api-reference/responses
pub fn routes() -> Router {
    Router::new()
        .route("/", post(create))
        .route("/{response_id}", get(get_))
        .route("/{response_id}", delete(delete_))
        .route("/{response_id}/input_items", get(input_items))
}

/// See: https://platform.openai.com/docs/api-reference/responses/create
#[axum::debug_handler]
async fn create(extract::Json(_): extract::Json<CreateResponse>) -> Json<Response> {
    Json(Response::default()) // TODO
}

/// See: https://platform.openai.com/docs/api-reference/responses/get
#[axum::debug_handler]
async fn get_(extract::Path(_): extract::Path<String>) -> Json<Response> {
    Json(Response::default()) // TODO
}

/// See: https://platform.openai.com/docs/api-reference/responses/delete
#[axum::debug_handler]
async fn delete_(extract::Path(_): extract::Path<String>) -> Json<Error> {
    Json(Error::default()) // TODO
}

/// See: https://platform.openai.com/docs/api-reference/responses/input-items
#[axum::debug_handler]
async fn input_items(extract::Path(_): extract::Path<String>) -> Json<ResponseItemList> {
    Json(ResponseItemList {
        object: "list".into(),
        data: vec![], // TODO
        first_id: "".into(),
        last_id: "".into(),
        has_more: false,
    })
}

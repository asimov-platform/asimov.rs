// This is free and unencumbered software released into the public domain.

//! Hugging Face helpers for model downloads with unified progress bar.
//!
//! ```no_run
//! use asimov_huggingface::{ensure_file, ensure_snapshot};
//!
//! let file = ensure_file("facebook/dinov2-base", "pytorch_model.bin").unwrap();
//! let dir = ensure_snapshot("julien-c/dummy-unknown", None).unwrap();
//! ```

mod ensure;
pub use ensure::*;

mod progress;
pub use progress::*;

use thiserror::Error;
use hf_hub::api::sync::ApiError;
use std::path::PathBuf;

#[derive(Debug, Error)]
pub enum HuggingfaceError {
    #[error("failed to access Hugging Face API: {0}")]
    Api(#[from] ApiError),

    #[error("snapshot is empty")]
    EmptySnapshot,

    #[error("failed to download file `{0}`")]
    Download(PathBuf),
}

pub type Result<T> = std::result::Result<T, HuggingfaceError>;


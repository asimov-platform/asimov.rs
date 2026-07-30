// This is free and unencumbered software released into the public domain.

use asimov_credit::CreditsError;
use thiserror::Error;

#[derive(Debug, Error)]
pub enum AccountBalanceError {
    #[error("HTTP request failed: {0}")]
    FailedRequest(#[from] reqwest::Error),

    #[error("unexpected HTTP response status: {0}")]
    UnexpectedResponse(reqwest::StatusCode),

    #[error("invalid HTTP response format: {0}")]
    InvalidResponseFormat(#[from] CreditsError),
}

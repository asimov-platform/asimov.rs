// This is free and unencumbered software released into the public domain.

use super::NodeHello;
use asimov_kb::BlobId;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Eq, Serialize, Deserialize, PartialEq)]
pub enum NodeResponse {
    Pong,

    Hello(NodeHello),

    Blob(BlobId),
}

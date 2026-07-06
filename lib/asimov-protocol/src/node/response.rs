// This is free and unencumbered software released into the public domain.

use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Serialize, Deserialize)]
pub enum NodeResponse {
    Ping,
}

// This is free and unencumbered software released into the public domain.

use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Eq, Serialize, Deserialize, PartialEq)]
pub enum NodeResponse {
    Ping,
}

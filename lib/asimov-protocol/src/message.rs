// This is free and unencumbered software released into the public domain.

use crate::PeerHello;
use alloc::{string::String, vec::Vec};
use asimov_kb::BlobId;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Eq, Serialize, Deserialize, PartialEq)]
#[non_exhaustive]
pub enum Message {
    Ping,

    Hello(PeerHello),

    Bye,

    List(Vec<String>),

    Blob(BlobId),
}

// This is free and unencumbered software released into the public domain.

use crate::TopicId;
use alloc::{format, string::String};
use rdf_hash::TermHash;
use rdf_model::HeapTerm;

#[derive(Debug)]
pub enum Topic {
    /// A handle topic
    Handle(String),
}

impl Topic {
    pub fn id(&self) -> TopicId {
        let term = HeapTerm::iri(self.to_uri());
        let term_hash = TermHash::from(term);
        TopicId::from(*term_hash.as_bytes())
    }

    pub fn to_uri(&self) -> String {
        match self {
            Self::Handle(handle) => format!("https://asimov.social/{}", handle),
        }
    }
}

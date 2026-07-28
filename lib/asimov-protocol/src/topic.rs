// This is free and unencumbered software released into the public domain.

use crate::TopicId;
use alloc::{format, string::String};
use asimov_id::{Handle, HandleError};
use core::{fmt::Display, str::FromStr};
use rdf_hash::TermHash;
use rdf_model::HeapTerm;

#[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub enum Topic {
    /// A handle topic
    Handle(Handle),
}

impl Topic {
    pub fn id(&self) -> TopicId {
        let term = HeapTerm::iri(self.to_uri());
        let term_hash = TermHash::from(term);
        TopicId::from(*term_hash.as_bytes())
    }

    pub fn to_uri(&self) -> String {
        match self {
            Self::Handle(handle) => handle.to_uri(),
        }
    }
}

impl Display for Topic {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "{}", self.to_uri())
    }
}

impl FromStr for Topic {
    type Err = HandleError;

    fn from_str(input: &str) -> Result<Self, Self::Err> {
        Ok(Topic::Handle(Handle::from_str(input)?))
    }
}

impl<T> From<&T> for Topic
where
    T: Clone + Into<Self>,
{
    fn from(t: &T) -> Self {
        t.clone().into()
    }
}

impl TryFrom<String> for Topic {
    type Error = HandleError;

    fn try_from(input: String) -> Result<Self, Self::Error> {
        Self::from_str(&input)
    }
}

impl From<&Topic> for String {
    fn from(input: &Topic) -> Self {
        input.to_uri()
    }
}

impl From<Handle> for Topic {
    fn from(input: Handle) -> Self {
        Topic::Handle(input)
    }
}

// This is free and unencumbered software released into the public domain.

use alloc::vec::Vec;
use serde::{Deserialize, Deserializer, Serialize};

type String = heapless::String<32>;

#[derive(Clone, Debug, Eq, Serialize, PartialEq)]
#[serde(untagged)]
pub enum NodeFeatureSet<'a> {
    Borrowed(&'a [&'a str]),
    Owned(Vec<String>),
}

impl<'a> NodeFeatureSet<'a> {
    pub fn iter(&'a self) -> FeatureIter<'a> {
        match self {
            NodeFeatureSet::Borrowed(slice) => FeatureIter::Borrowed(slice.iter()),
            NodeFeatureSet::Owned(vec) => FeatureIter::Owned(vec.iter()),
        }
    }
}

impl<'de, 'a> Deserialize<'de> for NodeFeatureSet<'a> {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let vec = Vec::<String>::deserialize(deserializer)?;
        Ok(NodeFeatureSet::Owned(vec))
    }
}

pub enum FeatureIter<'a> {
    Borrowed(core::slice::Iter<'a, &'a str>),
    Owned(core::slice::Iter<'a, String>),
}

impl<'a> Iterator for FeatureIter<'a> {
    type Item = &'a str;

    fn next(&mut self) -> Option<Self::Item> {
        match self {
            FeatureIter::Borrowed(iter) => iter.next().copied(),
            FeatureIter::Owned(iter) => iter.next().map(|s| s.as_str()),
        }
    }
}

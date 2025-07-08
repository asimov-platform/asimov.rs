// This is free and unencumbered software released into the public domain.

use crate::{
    Named,
    prelude::{Cow, String, fmt::Debug},
};
use asimov_sys::AsiModelManifest;

pub use asimov_core::model::ModelManifest;

#[derive(Debug)]
pub(crate) struct LocalModelManifest {
    inner: AsiModelManifest,
}

impl LocalModelManifest {
    pub fn new(inner: AsiModelManifest) -> Self {
        Self { inner }
    }
}

impl Named for LocalModelManifest {
    fn name(&self) -> Cow<str> {
        self.inner.name_lossy()
    }
}

impl ModelManifest for LocalModelManifest {}

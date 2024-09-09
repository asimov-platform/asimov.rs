// This is free and unencumbered software released into the public domain.

use crate::prelude::{fmt::Debug, Cow, Named, String};
use asimov_sys::AsiModelManifest;

#[stability::unstable]
pub trait ModelManifest: Named + Debug {}

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

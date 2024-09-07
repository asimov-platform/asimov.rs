// This is free and unencumbered software released into the public domain.

use crate::{
    prelude::{fmt::Debug, String},
    Named,
};
use asimov_sys::AsiModelManifest;

#[stability::unstable]
pub trait ModelManifest: Debug + Named {}

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
    fn name(&self) -> String {
        self.inner.name_lossy().into_owned()
    }
}

impl ModelManifest for LocalModelManifest {}

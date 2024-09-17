// This is free and unencumbered software released into the public domain.

use crate::{
    prelude::{fmt::Debug, Box, Cow, String},
    BlockDefinition, Named, Result,
};
use asimov_sys::AsiBlockUsage;

#[derive(Debug, Default)]
pub struct BlockUsage {
    pub(crate) inner: AsiBlockUsage,
}

impl BlockUsage {
    #[allow(unused)]
    pub fn new(name: &str, r#type: &str) -> Self {
        Self {
            inner: AsiBlockUsage::new(name, r#type),
        }
    }

    pub fn r#type(&self) -> Cow<str> {
        self.inner.type_lossy()
    }
}

impl From<AsiBlockUsage> for BlockUsage {
    fn from(inner: AsiBlockUsage) -> Self {
        Self { inner }
    }
}

impl Named for BlockUsage {
    fn name(&self) -> Cow<str> {
        self.inner.name_lossy()
    }
}

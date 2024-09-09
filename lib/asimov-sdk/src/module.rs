// This is free and unencumbered software released into the public domain.

use crate::prelude::{fmt::Debug, Cow, Named, String};
use asimov_sys::AsiModuleRegistration;

#[stability::unstable]
pub trait ModuleRegistration: Named + Debug {}

#[derive(Debug)]
pub(crate) struct StaticModuleRegistration {
    inner: AsiModuleRegistration,
}

impl StaticModuleRegistration {
    pub fn new(inner: AsiModuleRegistration) -> Self {
        Self { inner }
    }
}

impl Named for StaticModuleRegistration {
    fn name(&self) -> Cow<str> {
        self.inner.name_lossy()
    }
}

impl ModuleRegistration for StaticModuleRegistration {}

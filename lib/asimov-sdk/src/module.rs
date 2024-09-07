// This is free and unencumbered software released into the public domain.

use crate::{prelude::String, Named};
use asimov_sys::AsiModuleRegistration;

#[stability::unstable]
pub trait ModuleRegistration: Named {}

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
    fn name(&self) -> String {
        self.inner.name_lossy().into_owned()
    }
}

impl ModuleRegistration for StaticModuleRegistration {}

// This is free and unencumbered software released into the public domain.

use crate::{
    Named,
    prelude::{Cow, Result, String, fmt::Debug},
};
use asimov_sys::{AsiInstance, AsiModuleRegistration};

pub use asimov_core::module::ModuleRegistration;

#[derive(Debug)]
pub(crate) struct StaticModuleRegistration {
    #[allow(unused)]
    instance: AsiInstance,
    inner: AsiModuleRegistration,
}

impl StaticModuleRegistration {
    pub fn new(instance: AsiInstance, inner: AsiModuleRegistration) -> Self {
        Self { instance, inner }
    }
}

impl Named for StaticModuleRegistration {
    fn name(&self) -> Cow<str> {
        self.inner.name_lossy()
    }
}

impl ModuleRegistration for StaticModuleRegistration {
    fn is_enabled(&self) -> bool {
        true
    }

    fn enable(&mut self) -> Result<bool, ()> {
        Ok(false)
    }

    fn disable(&mut self) -> Result<bool, ()> {
        Ok(false)
    }
}

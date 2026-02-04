// This is free and unencumbered software released into the public domain.

use asimov_core::ModuleName;

pub trait ModuleNameIterator {
    fn next(&mut self) -> impl Future<Output = Option<ModuleName>> + Send;
}

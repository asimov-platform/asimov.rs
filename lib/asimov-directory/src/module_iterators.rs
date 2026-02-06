// This is free and unencumbered software released into the public domain.

use asimov_core::ModuleName;

/// An iterator over module names in a module directory.
pub trait ModuleNameIterator {
    fn next(&mut self) -> impl Future<Output = Option<ModuleName>> + Send;
}

/// An iterator over module names in a module directory.
pub trait BlockingModuleNameIterator {
    fn next(&mut self) -> Option<ModuleName>;
}

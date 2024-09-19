// This is free and unencumbered software released into the public domain.

use crate::Named;
use core::fmt::Debug;

pub trait ModuleRegistration: Named + Debug {
    fn is_enabled(&self) -> bool {
        true
    }

    fn enable(&mut self) -> Result<bool, ()>;

    fn disable(&mut self) -> Result<bool, ()>;
}

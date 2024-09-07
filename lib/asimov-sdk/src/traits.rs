// This is free and unencumbered software released into the public domain.

use crate::prelude::String;

pub trait Named {
    fn name(&self) -> String;
}

pub trait MaybeNamed {
    fn name(&self) -> Option<String>;
}

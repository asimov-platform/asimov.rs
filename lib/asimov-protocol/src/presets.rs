// This is free and unencumbered software released into the public domain.

use iroh::endpoint::{
    Builder,
    presets::{N0, Preset},
};

/// The default preset for ASIMOV.
#[derive(Copy, Clone, Default, Debug)]
pub struct DefaultPreset;

impl Preset for DefaultPreset {
    fn apply(self, builder: Builder) -> Builder {
        N0.apply(builder)
    }
}

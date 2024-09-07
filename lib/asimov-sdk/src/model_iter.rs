// This is free and unencumbered software released into the public domain.

use crate::{
    prelude::{Box, Vec},
    Error, LocalModelManifest, ModelManifest, Result,
};
use asimov_sys::{AsiInstance, AsiModelManifest};

pub(crate) struct ModelManifestIter {
    index: usize,
    elements: Vec<AsiModelManifest>,
}

impl TryFrom<AsiInstance> for ModelManifestIter {
    type Error = Error;

    fn try_from(_instance: AsiInstance) -> Result<Self> {
        todo!() // TODO
    }
}

impl From<Vec<AsiModelManifest>> for ModelManifestIter {
    fn from(elements: Vec<AsiModelManifest>) -> Self {
        Self { index: 0, elements }
    }
}

impl Iterator for ModelManifestIter {
    type Item = Box<dyn ModelManifest>;

    fn next(&mut self) -> Option<Self::Item> {
        if self.index < self.elements.len() {
            let element = self.elements[self.index];
            self.index += 1;
            Some(Box::new(LocalModelManifest::new(element)) as _)
        } else {
            None // end of iteration
        }
    }
}

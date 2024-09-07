// This is free and unencumbered software released into the public domain.

use crate::{
    prelude::{null_mut, vec, Box, Vec},
    Error, LocalModelManifest, ModelManifest, Result,
};
use asimov_sys::{asiEnumerateModels, AsiInstance, AsiModelManifest, AsiResult};

pub(crate) struct ModelManifestIter {
    #[allow(unused)]
    instance: AsiInstance,
    index: usize,
    elements: Vec<AsiModelManifest>,
}

impl ModelManifestIter {
    pub fn new(instance: AsiInstance, elements: Vec<AsiModelManifest>) -> Self {
        Self {
            instance,
            index: 0,
            elements,
        }
    }
}

impl TryFrom<AsiInstance> for ModelManifestIter {
    type Error = Error;

    fn try_from(instance: AsiInstance) -> Result<Self> {
        let mut count: u32 = 0;
        match unsafe { asiEnumerateModels(instance, 0, &mut count, null_mut()) } {
            AsiResult::ASI_SUCCESS => (),
            error => return Err(error.try_into().unwrap()),
        };

        let mut buffer = vec![AsiModelManifest::default(); count as _];
        match unsafe { asiEnumerateModels(instance, count, &mut count, buffer.as_mut_ptr()) } {
            AsiResult::ASI_SUCCESS => (),
            error => return Err(error.try_into().unwrap()),
        };

        Ok(Self::new(instance, buffer))
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

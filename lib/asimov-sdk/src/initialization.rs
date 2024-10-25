// This is free and unencumbered software released into the public domain.

use crate::Result;
use asimov_sys::{asiInitLibrary, AsiResult};

/// Initializes the SDK library.
///
/// It is crucial to call this function in entry point of your application,
/// BEFORE returning early, for example due to failed parsing of command line.
///
/// You MUST NOT call any other SDK library function until this function
/// returns from your initial call to it.
///
/// This function is idempotent: you can call it more than once without
/// ill effect.
pub fn initialize() -> Result<()> {
    match unsafe { asiInitLibrary(core::ptr::null(), core::ptr::null_mut()) } {
        AsiResult::ASI_SUCCESS => Ok(()),
        error => Err(error.try_into().unwrap()),
    }
}

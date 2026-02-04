// This is free and unencumbered software released into the public domain.

use alloc::string::String;
use asimov_core::Named;

/// A configuration profile.
pub trait ConfigProfile: Named {
    /// Returns the system prompt, if any, for the configuration profile.
    fn prompt(&self) -> Option<String> {
        None
    }
}

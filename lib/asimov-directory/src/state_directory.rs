// This is free and unencumbered software released into the public domain.

/// A state directory in the abstract.
pub trait StateDirectory {
    /// Checks if any modules are installed.
    fn has_modules(&self) -> bool {
        false
    }
}

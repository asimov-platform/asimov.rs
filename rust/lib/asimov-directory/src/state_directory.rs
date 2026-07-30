// This is free and unencumbered software released into the public domain.

/// A state directory in the abstract.
pub trait StateDirectory {
    /// Checks if any configurations are available.
    fn has_configs(&self) -> bool {
        false
    }

    /// Checks if any modules are installed.
    fn has_modules(&self) -> bool {
        false
    }

    /// Checks if any programs are installed.
    fn has_programs(&self) -> bool {
        false
    }
}

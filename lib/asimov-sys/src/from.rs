// This is free and unencumbered software released into the public domain.

#[cfg(feature = "std")]
impl From<std::io::Error> for AsiResult {
    fn from(error: std::io::Error) -> Self {
        use std::io::ErrorKind::*;
        match error.kind() {
            NotFound => Self::ASI_ERROR_PRECONDITION_VIOLATED,
            _ => Self::ASI_ERROR_NOT_IMPLEMENTED,
        }
    }
}

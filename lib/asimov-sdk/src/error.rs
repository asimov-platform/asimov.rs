// This is free and unencumbered software released into the public domain.

use std::convert::TryFrom;
use std::os::raw::c_int;

#[allow(unused)]
#[derive(Clone, Default, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub enum Error {
    #[default]
    NotImplemented,
    PreconditionViolated,
    HostMemoryExhausted,
    DeviceMemoryExhausted,
    Other(String),
}

#[allow(unused)]
impl Error {
    pub fn new(message: &str) -> Self {
        Self::Other(message.to_string())
    }
}

impl std::fmt::Debug for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            Self::NotImplemented => write!(f, "Error::NotImplemented"),
            Self::PreconditionViolated => write!(f, "Error::PreconditionViolated"),
            Self::HostMemoryExhausted => write!(f, "Error::HostMemoryExhausted"),
            Self::DeviceMemoryExhausted => write!(f, "Error::DeviceMemoryExhausted"),
            Self::Other(message) => write!(f, "Error::Other(\"{}\")", message),
        }
    }
}

impl std::fmt::Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            Self::NotImplemented => write!(f, "Not implemented"),
            Self::PreconditionViolated => write!(f, "Precondition violated"),
            Self::HostMemoryExhausted => write!(f, "Host memory exhausted"),
            Self::DeviceMemoryExhausted => write!(f, "Device memory exhausted"),
            Self::Other(message) => write!(f, "{}", message),
        }
    }
}

impl std::error::Error for Error {}

impl From<std::io::Error> for Error {
    fn from(error: std::io::Error) -> Self {
        Self::Other(error.to_string())
    }
}

impl From<std::num::ParseFloatError> for Error {
    fn from(error: std::num::ParseFloatError) -> Self {
        Self::Other(error.to_string())
    }
}

impl From<std::num::ParseIntError> for Error {
    fn from(error: std::num::ParseIntError) -> Self {
        Self::Other(error.to_string())
    }
}

impl From<std::str::Utf8Error> for Error {
    fn from(error: std::str::Utf8Error) -> Self {
        Self::Other(error.to_string())
    }
}

impl From<std::string::FromUtf8Error> for Error {
    fn from(error: std::string::FromUtf8Error) -> Self {
        Self::Other(error.to_string())
    }
}

impl From<std::string::FromUtf16Error> for Error {
    fn from(error: std::string::FromUtf16Error) -> Self {
        Self::Other(error.to_string())
    }
}

impl From<std::ffi::CString> for Error {
    fn from(error: std::ffi::CString) -> Self {
        Self::Other(error.to_string_lossy().to_string())
    }
}

impl From<std::ffi::FromBytesWithNulError> for Error {
    fn from(error: std::ffi::FromBytesWithNulError) -> Self {
        Self::Other(error.to_string())
    }
}

impl From<std::ffi::IntoStringError> for Error {
    fn from(error: std::ffi::IntoStringError) -> Self {
        Self::Other(error.to_string())
    }
}

impl From<std::ffi::NulError> for Error {
    fn from(error: std::ffi::NulError) -> Self {
        Self::Other(error.to_string())
    }
}

impl From<std::ffi::OsString> for Error {
    fn from(error: std::ffi::OsString) -> Self {
        Self::Other(error.to_string_lossy().to_string())
    }
}

impl TryFrom<c_int> for Error {
    type Error = ();

    fn try_from(code: c_int) -> Result<Self, Self::Error> {
        use asimov_sys::*;
        #[allow(non_upper_case_globals)]
        match code {
            AsiResult_ASI_ERROR_NOT_IMPLEMENTED => Ok(Error::NotImplemented),
            AsiResult_ASI_ERROR_PRECONDITION_VIOLATED => Ok(Error::PreconditionViolated),
            AsiResult_ASI_ERROR_HOST_MEMORY_EXHAUSTED => Ok(Error::HostMemoryExhausted),
            AsiResult_ASI_ERROR_DEVICE_MEMORY_EXHAUSTED => Ok(Error::DeviceMemoryExhausted),
            _ => Ok(Error::Other(format!("ASI_ERROR_{}", code))),
        }
    }
}

impl TryFrom<Error> for c_int {
    type Error = ();

    fn try_from(error: Error) -> Result<Self, Self::Error> {
        use asimov_sys::*;
        match error {
            Error::NotImplemented => Ok(AsiResult_ASI_ERROR_NOT_IMPLEMENTED),
            Error::PreconditionViolated => Ok(AsiResult_ASI_ERROR_PRECONDITION_VIOLATED),
            Error::HostMemoryExhausted => Ok(AsiResult_ASI_ERROR_HOST_MEMORY_EXHAUSTED),
            Error::DeviceMemoryExhausted => Ok(AsiResult_ASI_ERROR_DEVICE_MEMORY_EXHAUSTED),
            Error::Other(_) => Ok(0x7FFFFFFF),
        }
    }
}

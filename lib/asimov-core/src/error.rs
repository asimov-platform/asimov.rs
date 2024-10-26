// This is free and unencumbered software released into the public domain.

use alloc::{
    format,
    string::{FromUtf16Error, FromUtf8Error, String, ToString},
};
use asimov_sys::AsiResult;
use core::{
    convert::TryFrom,
    ffi::{c_int, FromBytesWithNulError},
    fmt,
    num::{ParseFloatError, ParseIntError},
    str::Utf8Error,
};

pub type Result<T, E = Error> = core::result::Result<T, E>;

#[allow(unused)]
#[derive(Clone, Debug, Default, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub enum Error {
    TimeoutExpired,
    ExitRequested,
    #[default]
    NotImplemented,
    PreconditionViolated,
    HostMemoryExhausted,
    DeviceMemoryExhausted,
    SizeInsufficient,
    Other(String),
}

#[allow(unused)]
impl Error {
    pub fn new(message: &str) -> Self {
        Self::Other(message.to_string())
    }
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Self::TimeoutExpired => write!(f, "Timeout expired"),
            Self::ExitRequested => write!(f, "Exit requested"),
            Self::NotImplemented => write!(f, "Not implemented"),
            Self::PreconditionViolated => write!(f, "Precondition violated"),
            Self::HostMemoryExhausted => write!(f, "Host memory exhausted"),
            Self::DeviceMemoryExhausted => write!(f, "Device memory exhausted"),
            Self::SizeInsufficient => write!(f, "Size insufficient"),
            Self::Other(message) => write!(f, "{}", message),
        }
    }
}

impl core::error::Error for Error {}

impl From<ParseFloatError> for Error {
    fn from(error: ParseFloatError) -> Self {
        Self::Other(error.to_string())
    }
}

impl From<ParseIntError> for Error {
    fn from(error: ParseIntError) -> Self {
        Self::Other(error.to_string())
    }
}

impl From<Utf8Error> for Error {
    fn from(error: Utf8Error) -> Self {
        Self::Other(error.to_string())
    }
}

impl From<FromUtf8Error> for Error {
    fn from(error: FromUtf8Error) -> Self {
        Self::Other(error.to_string())
    }
}

impl From<FromUtf16Error> for Error {
    fn from(error: FromUtf16Error) -> Self {
        Self::Other(error.to_string())
    }
}

impl From<FromBytesWithNulError> for Error {
    fn from(error: FromBytesWithNulError) -> Self {
        Self::Other(error.to_string())
    }
}

#[cfg(feature = "std")]
impl From<std::ffi::CString> for Error {
    fn from(error: std::ffi::CString) -> Self {
        Self::Other(error.to_string_lossy().to_string())
    }
}

#[cfg(feature = "std")]
impl From<std::ffi::IntoStringError> for Error {
    fn from(error: std::ffi::IntoStringError) -> Self {
        Self::Other(error.to_string())
    }
}

#[cfg(feature = "std")]
impl From<std::ffi::NulError> for Error {
    fn from(error: std::ffi::NulError) -> Self {
        Self::Other(error.to_string())
    }
}

#[cfg(feature = "std")]
impl From<std::ffi::OsString> for Error {
    fn from(error: std::ffi::OsString) -> Self {
        Self::Other(error.to_string_lossy().to_string())
    }
}

#[cfg(feature = "std")]
impl From<std::io::Error> for Error {
    fn from(error: std::io::Error) -> Self {
        Self::Other(error.to_string())
    }
}

impl TryFrom<AsiResult> for Error {
    type Error = ();

    fn try_from(input: AsiResult) -> Result<Self, Self::Error> {
        use AsiResult::*;
        Ok(match input {
            ASI_SUCCESS => return Err(()),
            ASI_TIMEOUT_EXPIRED => Self::TimeoutExpired,
            ASI_EXIT_REQUESTED => Self::ExitRequested,
            ASI_ERROR_NOT_IMPLEMENTED => Self::NotImplemented,
            ASI_ERROR_PRECONDITION_VIOLATED => Self::PreconditionViolated,
            ASI_ERROR_HOST_MEMORY_EXHAUSTED => Self::HostMemoryExhausted,
            ASI_ERROR_DEVICE_MEMORY_EXHAUSTED => Self::DeviceMemoryExhausted,
            ASI_ERROR_SIZE_INSUFFICIENT => Self::SizeInsufficient,
        })
    }
}

impl TryFrom<c_int> for Error {
    type Error = ();

    fn try_from(code: c_int) -> Result<Self, Self::Error> {
        match AsiResult::try_from(code) {
            Ok(result) => result.try_into(),
            Err(_) => Ok(Error::Other(format!("ASI_ERROR_{}", code))),
        }
    }
}

impl TryFrom<Error> for c_int {
    type Error = ();

    fn try_from(error: Error) -> Result<Self, Self::Error> {
        use AsiResult::*;
        Ok(match error {
            Error::TimeoutExpired => ASI_TIMEOUT_EXPIRED,
            Error::ExitRequested => ASI_EXIT_REQUESTED,
            Error::NotImplemented => ASI_ERROR_NOT_IMPLEMENTED,
            Error::PreconditionViolated => ASI_ERROR_PRECONDITION_VIOLATED,
            Error::HostMemoryExhausted => ASI_ERROR_HOST_MEMORY_EXHAUSTED,
            Error::DeviceMemoryExhausted => ASI_ERROR_DEVICE_MEMORY_EXHAUSTED,
            Error::SizeInsufficient => ASI_ERROR_SIZE_INSUFFICIENT,
            Error::Other(_) => return Err(()),
        } as c_int)
    }
}

// This is free and unencumbered software released into the public domain.

use crate::prelude::{
    c_int, fmt, format, FromBytesWithNulError, FromUtf16Error, FromUtf8Error, ParseFloatError,
    ParseIntError, String, ToString, TryFrom, Utf8Error,
};

#[cfg(feature = "std")]
extern crate std;

#[allow(unused)]
#[derive(Clone, Debug, Default, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub enum Error {
    TimeoutExpired,
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
            Self::NotImplemented => write!(f, "Not implemented"),
            Self::PreconditionViolated => write!(f, "Precondition violated"),
            Self::HostMemoryExhausted => write!(f, "Host memory exhausted"),
            Self::DeviceMemoryExhausted => write!(f, "Device memory exhausted"),
            Self::SizeInsufficient => write!(f, "Size insufficient"),
            Self::Other(message) => write!(f, "{}", message),
        }
    }
}

#[cfg(feature = "std")]
impl std::error::Error for Error {}

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

impl TryFrom<c_int> for Error {
    type Error = ();

    fn try_from(code: c_int) -> Result<Self, Self::Error> {
        use asimov_sys::*;
        let Ok(result) = AsiResult::try_from(code) else {
            return Ok(Error::Other(format!("ASI_ERROR_{}", code)));
        };
        Ok(match result {
            AsiResult::ASI_SUCCESS => return Err(()),
            AsiResult::ASI_TIMEOUT_EXPIRED => Error::TimeoutExpired,
            AsiResult::ASI_ERROR_NOT_IMPLEMENTED => Error::NotImplemented,
            AsiResult::ASI_ERROR_PRECONDITION_VIOLATED => Error::PreconditionViolated,
            AsiResult::ASI_ERROR_HOST_MEMORY_EXHAUSTED => Error::HostMemoryExhausted,
            AsiResult::ASI_ERROR_DEVICE_MEMORY_EXHAUSTED => Error::DeviceMemoryExhausted,
            AsiResult::ASI_ERROR_SIZE_INSUFFICIENT => Error::SizeInsufficient,
        })
    }
}

impl TryFrom<Error> for c_int {
    type Error = ();

    fn try_from(error: Error) -> Result<Self, Self::Error> {
        use asimov_sys::*;
        Ok(match error {
            Error::TimeoutExpired => AsiResult::ASI_TIMEOUT_EXPIRED,
            Error::NotImplemented => AsiResult::ASI_ERROR_NOT_IMPLEMENTED,
            Error::PreconditionViolated => AsiResult::ASI_ERROR_PRECONDITION_VIOLATED,
            Error::HostMemoryExhausted => AsiResult::ASI_ERROR_HOST_MEMORY_EXHAUSTED,
            Error::DeviceMemoryExhausted => AsiResult::ASI_ERROR_DEVICE_MEMORY_EXHAUSTED,
            Error::SizeInsufficient => AsiResult::ASI_ERROR_SIZE_INSUFFICIENT,
            Error::Other(_) => return Err(()),
        } as c_int)
    }
}

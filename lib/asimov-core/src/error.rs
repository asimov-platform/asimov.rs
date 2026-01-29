// This is free and unencumbered software released into the public domain.

use alloc::string::{FromUtf8Error, FromUtf16Error, String, ToString};
use core::{
    ffi::FromBytesWithNulError,
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

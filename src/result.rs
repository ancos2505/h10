use std::{
    error::Error,
    fmt::Display,
    io::Error as IoError,
    num::{ParseFloatError, ParseIntError},
    time::SystemTimeError,
};

pub(crate) type ServerResult<T> = Result<T, H10ServerError>;

#[derive(Debug)]
pub enum H10ServerError {
    VersionNotSupported,
    MethodNotSupported,
    InvalidInputData(String),
    ParseFloatError(ParseFloatError),
    SystemTimeError(SystemTimeError),
    ParseIntError(ParseIntError),
    IoError(IoError),
    Custom(String),
}

impl From<ParseFloatError> for H10ServerError {
    fn from(error: ParseFloatError) -> Self {
        Self::ParseFloatError(error)
    }
}

impl From<SystemTimeError> for H10ServerError {
    fn from(error: SystemTimeError) -> Self {
        Self::SystemTimeError(error)
    }
}

impl From<ParseIntError> for H10ServerError {
    fn from(error: ParseIntError) -> Self {
        Self::ParseIntError(error)
    }
}

impl From<IoError> for H10ServerError {
    fn from(error: IoError) -> Self {
        Self::IoError(error)
    }
}

impl Display for H10ServerError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}", self)
    }
}

impl Error for H10ServerError {}

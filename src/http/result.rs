use std::{
    error::Error,
    fmt::Display,
    io::Error as IoError,
    num::{ParseFloatError, ParseIntError},
    time::SystemTimeError,
};

pub type H10LibResult<T> = Result<T, H10LibError>;

#[derive(Debug)]
pub enum H10LibError {
    VersionNotSupported,
    MethodNotSupported,
    InvalidInputData(String),
    ParseFloatError(ParseFloatError),
    SystemTimeError(SystemTimeError),
    ParseIntError(ParseIntError),
    IoError(IoError),
    Custom(String),
}

impl From<ParseFloatError> for H10LibError {
    fn from(error: ParseFloatError) -> Self {
        Self::ParseFloatError(error)
    }
}

impl From<SystemTimeError> for H10LibError {
    fn from(error: SystemTimeError) -> Self {
        Self::SystemTimeError(error)
    }
}

impl From<ParseIntError> for H10LibError {
    fn from(error: ParseIntError) -> Self {
        Self::ParseIntError(error)
    }
}

impl From<IoError> for H10LibError {
    fn from(error: IoError) -> Self {
        Self::IoError(error)
    }
}

impl Display for H10LibError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}", self)
    }
}

impl Error for H10LibError {}

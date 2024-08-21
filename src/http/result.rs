use std::{
    error::Error,
    fmt::Display,
    io::Error as IoError,
    num::{ParseFloatError, ParseIntError},
    time::SystemTimeError,
};

use super::status_code::StatusCode;

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

impl From<H10LibError> for StatusCode {
    fn from(value: H10LibError) -> Self {
        match value {
            H10LibError::MethodNotSupported
            | H10LibError::InvalidInputData(_)
            | H10LibError::VersionNotSupported => StatusCode::BadRequest,
            H10LibError::ParseFloatError(_) => StatusCode::InternalServerError,
            H10LibError::SystemTimeError(_) => StatusCode::InternalServerError,
            H10LibError::ParseIntError(_) => StatusCode::InternalServerError,
            H10LibError::IoError(_) => StatusCode::InternalServerError,
            H10LibError::Custom(_) => StatusCode::InternalServerError,
        }
    }
}

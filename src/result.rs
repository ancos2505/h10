use std::{
    error::Error,
    fmt::Display,
    io::Error as IoError,
    num::{ParseFloatError, ParseIntError},
};

pub(crate) type AppResult<T> = Result<T, H10ServerError>;

#[derive(Debug)]
pub struct H10ServerError(pub String);

impl From<ParseFloatError> for H10ServerError {
    fn from(error: ParseFloatError) -> Self {
        Self(error.to_string())
    }
}

impl From<ParseIntError> for H10ServerError {
    fn from(error: ParseIntError) -> Self {
        Self(error.to_string())
    }
}

impl From<IoError> for H10ServerError {
    fn from(error: IoError) -> Self {
        Self(error.to_string())
    }
}

impl Display for H10ServerError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl Error for H10ServerError {}

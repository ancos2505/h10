use std::{fmt::Display, str::FromStr};

use crate::http::result::H10LibError;

#[derive(Debug, Default, PartialEq, Eq)]
pub enum LogLevel {
    #[default]
    Error,
    Warn,
    Info,
    Debug,
    Trace,
}
impl FromStr for LogLevel {
    type Err = H10LibError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let level = match s {
            "ERROR" => Self::Error,
            "WARN" => Self::Warn,
            "INFO" => Self::Info,
            "DEBUG" => Self::Debug,
            "TRACE" => Self::Trace,
            _ => return Err(H10LibError::Custom("Invalid LogLevel".into())),
        };
        Ok(level)
    }
}

impl Display for LogLevel {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let s = match self {
            Self::Error => "ERROR",
            Self::Warn => "WARN",
            Self::Info => "INFO",
            Self::Debug => "DEBUG",
            Self::Trace => "TRACE",
        };
        write!(f, "{s}")
    }
}

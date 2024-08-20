use std::{fmt::Display, str::FromStr};

use crate::result::H10ServerError;

/// ### HTTP Method
///
/// Reference: https://www.rfc-editor.org/rfc/rfc1945.html#section-5.1.1
#[derive(Debug)]
pub enum Method {
    Get,
    Head,
    Post,
}

impl FromStr for Method {
    type Err = H10ServerError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let method = match s {
            "GET" => Self::Get,
            "HEAD" => Self::Head,
            "POST" => Self::Post,
            _ => return Err(H10ServerError("Invalid HTTP Method".to_owned())),
        };
        Ok(method)
    }
}

impl Display for Method {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let output = match self {
            Self::Get => "GET".to_owned(),
            Self::Head => "HEAD".to_owned(),
            Self::Post => "POST".to_owned(),
        };
        write!(f, "{output}")
    }
}

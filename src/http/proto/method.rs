use std::{fmt::Display, str::FromStr};

use crate::result::H10ServerError;

pub enum Method {
    Get,
    Head,
    Post,
    Put,
    Delete,
    Connect,
    Options,
    Trace,
}

impl FromStr for Method {
    type Err = H10ServerError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let method = match s {
            "GET" => Self::Get,
            "HEAD" => Self::Head,
            "POST" => Self::Post,
            "PUT" => Self::Put,
            "DELETE" => Self::Delete,
            "CONNECT" => Self::Connect,
            "OPTIONS" => Self::Options,
            "TRACE" => Self::Trace,
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
            Self::Put => "PUT".to_owned(),
            Self::Delete => "DELETE".to_owned(),
            Self::Connect => "CONNECT".to_owned(),
            Self::Options => "OPTIONS".to_owned(),
            Self::Trace => "TRACE".to_owned(),
        };
        write!(f, "{output}")
    }
}

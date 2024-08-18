use std::{
    collections::HashMap,
    fmt::{Debug, Display},
    str::FromStr,
};

use crate::result::H10ServerError;

use super::{
    status_code::{StatusCode, ValidCode},
    version::Version,
};

pub const URL_MAX_LENGTH: usize = 4096;

/*
< HTTP/1.0 404 File not found
< Server: SimpleHTTP/0.6 Python/3.11.2
< Date: Sun, 18 Aug 2024 10:28:41 GMT
< Connection: close
< Content-Type: text/html;charset=utf-8
< Content-Length: 335

*/
#[derive(Debug, PartialEq, Eq)]
pub struct Response<'a, T: Debug + Display + ValidCode> {
    http_version: Version,
    status: StatusCode<T>,
    headers: HashMap<String, String>,
    body: Option<Body<'a>>,
}

impl<T: Debug + Display + ValidCode> Display for Response<'_, T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut output = "".to_owned();
        output.push_str(&self.http_version.to_string());
        output.push(' ');
        output.push_str(&self.status.to_string());
        output.push_str("\r\n");

        for (key, value) in self.headers.iter() {
            output.push_str(&key);
            output.push(' ');
            output.push_str(&value);
            output.push_str("\r\n");
        }

        output.push_str("\r\n");

        write!(f, "{}", output)
    }
}
impl<'a, T: Debug + Display + ValidCode> Response<'a, T> {
    pub fn new(status: StatusCode<T>) -> Response<'a, T> {
        Response {
            http_version: Default::default(),
            status,
            headers: Default::default(),
            body: Default::default(),
        }
    }
}

#[derive(Debug, PartialEq, Eq)]
pub struct UrlPath(String);
impl FromStr for UrlPath {
    type Err = H10ServerError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if s.chars().count() < URL_MAX_LENGTH {
            for c in s.chars() {
                if c.is_alphanumeric() || c.is_ascii_punctuation() {
                    continue;
                } else {
                    return Err(H10ServerError("Invalid char for URL".into()));
                }
            }
            Ok(Self(s.to_owned()))
        } else {
            Err(H10ServerError("Path is too large".into()))
        }
    }
}

#[derive(Debug, PartialEq, Eq)]
pub struct Body<'a>(&'a str);

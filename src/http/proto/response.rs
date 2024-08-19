use std::{
    collections::HashMap,
    fmt::{Debug, Display},
    str::FromStr,
};

use html_rs::Html;

use crate::result::H10ServerError;

use super::{
    headers::{HttpHeader, IntoHeader},
    status_code::{StatusCode, ValidCode},
    version::Version,
};

pub const URL_MAX_LENGTH: usize = 4096;

#[derive(Debug)]
pub struct Response<T: Debug + Display + ValidCode> {
    http_version: Version,
    status: StatusCode<T>,
    headers: HashMap<String, String>,
    body: Option<String>,
}

impl<T: Debug + Display + ValidCode> Display for Response<T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut output = "".to_owned();
        output.push_str(&self.http_version.to_string());
        output.push(' ');
        output.push_str(&self.status.to_string());
        output.push_str("\r\n");

        for (name, value) in self.headers.iter() {
            output.push_str(format!("{name}: {value}").as_str());
            output.push_str("\r\n");
        }

        output.push_str("\r\n");

        if let Some(body) = &self.body {
            output.push_str(body.as_str());
        }

        output.push_str("\r\n\r\n");

        write!(f, "{}", output)
    }
}
impl<T: Debug + Display + ValidCode> Response<T> {
    pub fn new(status: StatusCode<T>) -> Response<T> {
        Response {
            http_version: Default::default(),
            status,
            headers: Default::default(),
            body: Default::default(),
        }
    }
    pub fn header<H: IntoHeader>(mut self, header: H) -> Response<T> {
        let HttpHeader { name, value } = header.into_header();
        self.headers.insert(name, value);
        Response {
            http_version: self.http_version,
            status: self.status,
            headers: self.headers,
            body: self.body,
        }
    }
    pub fn body<B: ToString>(self, body: B) -> Response<T> {
        use crate::http::proto::headers::ContentLength;
        let body = body.to_string();

        let response = self.header(ContentLength::length(body.len() + 4));

        Response {
            http_version: response.http_version,
            status: response.status,
            headers: response.headers,
            body: Some(body),
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
pub struct Body(String);

impl From<Html<'_>> for Body {
    fn from(value: Html<'_>) -> Self {
        Self(value.to_string())
    }
}

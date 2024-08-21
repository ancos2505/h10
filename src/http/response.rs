use std::{
    collections::BTreeMap,
    fmt::{Debug, Display},
    str::FromStr,
};

use html_rs::Html;

use crate::http::result::H10LibError;

use super::{
    headers::{Connection, HttpHeader, IntoHeader},
    status_code::StatusCode,
    version::Version,
};

pub const URL_MAX_LENGTH: usize = 4096;

#[derive(Debug)]
pub struct Response {
    http_version: Version,
    pub status: StatusCode,
    headers: BTreeMap<String, String>,
    body: Option<String>,
}

impl Display for Response {
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
impl Response {
    pub fn new(status: StatusCode) -> Self {
        Self {
            http_version: Default::default(),
            status,
            headers: Default::default(),
            body: Default::default(),
        }
    }
    pub fn header<H: IntoHeader>(mut self, header: H) -> Self {
        let HttpHeader { name, value } = header.into_header();
        self.headers.insert(name, value);
        Self {
            http_version: self.http_version,
            status: self.status,
            headers: self.headers,
            body: self.body,
        }
    }
    pub fn body<B: ToString>(self, body: B) -> Self {
        use crate::http::headers::ContentLength;
        let body = body.to_string();

        let response = self.header(ContentLength::length(body.len() + 4));

        Self {
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
    type Err = H10LibError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if s.chars().count() < URL_MAX_LENGTH {
            for c in s.chars() {
                if c.is_alphanumeric() || c.is_ascii_punctuation() {
                    continue;
                } else {
                    return Err(H10LibError::InvalidInputData("Invalid char for URL".into()));
                }
            }
            Ok(Self(s.to_owned()))
        } else {
            Err(H10LibError::InvalidInputData("Path is too large".into()))
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

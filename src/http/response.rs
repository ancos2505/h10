use std::{
    fmt::{Debug, Display},
    str::FromStr,
};

use crate::{constants::URL_MAX_LENGTH, http::result::H10LibError};

use super::{
    body::Body,
    headers::{Headers, IntoHeader},
    status_code::StatusCode,
    version::Version,
};

#[derive(Debug)]
pub struct Response {
    http_version: Version,
    pub status: StatusCode,
    headers: Headers,
    body: Option<Body>,
}

impl Display for Response {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut output = "".to_owned();
        output.push_str(&self.http_version.to_string());
        output.push(' ');
        output.push_str(&self.status.to_string());
        output.push_str("\r\n");

        for header_entry in self.headers.iter() {
            output.push_str(&header_entry.to_string());
            output.push_str("\r\n");
        }

        if let Some(body) = &self.body {
            output.push_str("\r\n");
            output.push_str(&body.to_string());
            output.push_str("\n");
        }

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
    pub fn add_header<H: IntoHeader>(mut self, header: H) -> Self {
        let header_entry = header.into_header();
        self.headers.add(header_entry);
        Self {
            http_version: self.http_version,
            status: self.status,
            headers: self.headers,
            body: self.body,
        }
    }
    pub fn body<B: AsRef<str>>(self, body: B) -> Self {
        use crate::http::headers::ContentLength;
        let body = Body::new_unchecked(body.as_ref());
        let response = self.add_header(ContentLength::length(body.len() + 1));

        Self {
            http_version: response.http_version,
            status: response.status,
            headers: response.headers,
            body: Some(body.into()),
        }
    }
}

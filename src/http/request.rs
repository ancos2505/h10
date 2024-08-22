pub mod parser;
#[cfg(test)]
mod tests;

use std::{collections::BTreeMap, str::FromStr};

use crate::{
    constants::MAX_REQUEST_LENGTH,
    http::result::{H10LibError, H10LibResult},
};

use super::{method::Method, url::UrlParts, version::Version};

// TODO: Optimize data types to fast fields parsing
#[derive(Debug, Default)]
pub struct Request {
    pub http_version: Version,
    method: Method,
    pub url_parts: UrlParts,
    headers: Option<BTreeMap<String, String>>,
    body: Option<Body>,
}
impl Request {
    pub fn parse<S: AsRef<str>>(to_s: S) -> H10LibResult<Self> {
        let input = to_s.as_ref();
        if input.len() > MAX_REQUEST_LENGTH {
            return Err(H10LibError::InvalidInputData(format!(
                "Request size is larger than expected. MAX: {} Bytes",
                MAX_REQUEST_LENGTH
            )));
        }
        let mut raw_request = input.split("\r\n\r\n");

        let headers = raw_request.next().ok_or(H10LibError::InvalidInputData(
            "HTTP headers not found".into(),
        ))?;

        let mut iter_headers = headers.split("\r\n");

        let method_line = iter_headers.next().ok_or(H10LibError::InvalidInputData(
            "HTTP headers not found".into(),
        ))?;
        let mut iter = method_line.split(" ");
        let method_str = iter.next().ok_or(H10LibError::MethodNotSupported)?;

        let path_str = iter
            .next()
            .ok_or(H10LibError::InvalidInputData("Url Path not found".into()))?;

        let version_str = iter.next().ok_or(H10LibError::VersionNotSupported)?;

        let headers = iter_headers.next().map(|raw_headers| {
            raw_headers
                .lines()
                .map(|line| {
                    line.split_once(":")
                        .map(|(key, value)| (key.trim(), value.trim()))
                        .map(|(key, value)| (key.to_string(), value.to_string()))
                        .into_iter()
                        .collect()
                })
                .collect()
        });

        let body = if let Some(raw_body) = raw_request.next() {
            Some(raw_body.parse::<Body>()?)
        } else {
            None
        };

        let request = Self {
            http_version: version_str.parse::<Version>()?,
            method: method_str.parse::<Method>()?,
            url_parts: UrlParts::parse(path_str)?,
            headers,
            body,
        };

        Ok(request)
    }
}

/// ### Request Body
/// Should compilant with RFC 1867 - Form-based File Upload in HTML
///
/// Reference: https://www.rfc-editor.org/rfc/rfc1867
///
#[derive(Debug, PartialEq, Eq)]
pub struct Body(String);

impl FromStr for Body {
    type Err = H10LibError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let collected: String = s.chars().filter(|c| *c as u8 != 0).collect();
        Ok(Self(collected.to_owned()))
    }
}

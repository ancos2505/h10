use std::collections::BTreeMap;

use crate::{
    http::result::{H10LibError, H10LibResult},
    MAX_REQUEST_LENGTH,
};

use super::{method::Method, url::UrlParts, version::Version};

// TODO: Optimize data types to fast fields parsing
#[derive(Debug, Default)]
pub struct Request {
    pub http_version: Version,
    method: Method,
    pub url_parts: UrlParts,
    // TODO
    headers: Option<BTreeMap<String, String>>,
    // TODO
    body: Option<String>,
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
        let mut raw_request = input.split("\r\n");

        let raw_method_line = raw_request.next().ok_or(H10LibError::InvalidInputData(
            "HTTP Preamble not found".into(),
        ))?;

        let mut iter = raw_method_line.split(" ");
        let method_str = iter.next().ok_or(H10LibError::MethodNotSupported)?;

        let path_str = iter
            .next()
            .ok_or(H10LibError::InvalidInputData("Url Path not found".into()))?;

        let version_str = iter.next().ok_or(H10LibError::VersionNotSupported)?;

        let headers = raw_request.next().map(|raw_headers| {
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

        let body = raw_request.next().map(|s| s.to_string());

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

use std::{borrow::Cow, collections::BTreeMap};

use crate::{
    http::result::{H10LibError, H10LibResult},
    MAX_REQUEST_LENGTH,
};

use super::{method::Method, url::UrlParts, version::Version};

// TODO: Optimize data types to fast fields parsing
#[derive(Debug, Default)]
pub struct Request<'a> {
    pub http_version: Version,
    method: Method,
    pub url_parts: UrlParts,
    // TODO
    headers: Option<BTreeMap<&'a str, &'a str>>,
    // TODO
    body: Option<Body<'a>>,
}
impl Request<'_> {
    pub fn parse<S: AsRef<str>>(to_s: S) -> H10LibResult<Self> {
        let input = to_s.as_ref();
        if input.len() > MAX_REQUEST_LENGTH {
            return Err(H10LibError::InvalidInputData(format!(
                "Request size is larger than expected. MAX: {} Bytes",
                MAX_REQUEST_LENGTH
            )));
        }

        let preamble = input
            .split("\r\n")
            .next()
            .ok_or(H10LibError::InvalidInputData(
                "HTTP Preamble not found".into(),
            ))?;

        let mut iter = preamble.split(" ");
        let method_str = iter.next().ok_or(H10LibError::MethodNotSupported)?;

        let path_str = iter
            .next()
            .ok_or(H10LibError::InvalidInputData("Url Path not found".into()))?;

        let version_str = iter.next().ok_or(H10LibError::VersionNotSupported)?;

        let request = Self {
            http_version: version_str.parse::<Version>()?,
            method: method_str.parse::<Method>()?,
            url_parts: UrlParts::parse(path_str)?,
            headers: Default::default(),
            // TODO
            body: Default::default(),
        };

        Ok(request)
    }
}

#[derive(Debug)]
pub struct Body<'a>(&'a str);

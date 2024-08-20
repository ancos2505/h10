use std::{borrow::Cow, collections::BTreeMap};

use crate::result::{H10ServerError, ServerResult};

use super::{method::Method, url::UrlParts, version::Version};

const TEN_MEGABYTES: usize = 1024 * 1024 * 10;

// TODO: Optimize data types to fast fields parsing
#[derive(Debug, Default)]
pub struct Request<'a> {
    http_version: Option<Version>,
    method: Option<Method>,
    pub path: Option<UrlParts>,
    // TODO
    headers: Option<BTreeMap<&'a str, &'a str>>,
    // TODO
    body: Option<Body<'a>>,
}
impl<'a> Request<'a> {
    pub fn parse(input: Cow<'a, str>) -> ServerResult<Self> {
        if input.len() > TEN_MEGABYTES {
            return Err(H10ServerError("Request is larger than 10 MBytes".into()));
        }
        let mut request = Request::default();

        let preamble = input
            .split("\r\n")
            .next()
            .ok_or(H10ServerError("HTTP Preamble not found".into()))?;

        let mut iter = preamble.split(" ");
        let method_str = iter
            .next()
            .ok_or(H10ServerError("Method not found".into()))?;

        let path_str = iter
            .next()
            .ok_or(H10ServerError("Url Path not found".into()))?;

        let version_str = iter
            .next()
            .ok_or(H10ServerError("Version not found".into()))?;

        request.method = Some(method_str.parse::<Method>()?);
        request.path = Some(UrlParts::parse(path_str)?);
        request.http_version = Some(version_str.parse::<Version>()?);

        Ok(request)
    }
}

#[derive(Debug)]
pub struct Body<'a>(&'a str);

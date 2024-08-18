use std::collections::BTreeMap;

use super::{method::Method, version::Version};

pub struct Request<'a> {
    http_version: Version,
    method: Method,
    path: UrlPath<'a>,
    headers: BTreeMap<String, String>,
    body: Option<Body<'a>>,
}

pub struct UrlPath<'a>(&'a str);

pub struct Body<'a>(&'a str);

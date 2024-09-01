use crate::http::{headers::IntoHeader, method::Method, url_path::UrlPath, version::Version};

use super::{Body, Headers, QsEntry, QueryString, Request};

#[derive(Debug)]
pub struct RequestBuilder;

impl RequestBuilder {
    pub fn version(http_version: Version) -> Step1 {
        Step1 { http_version }
    }
}

#[derive(Debug)]
pub struct Step1 {
    http_version: Version,
}

impl Step1 {
    pub fn method(self, method: Method) -> Step2 {
        let Self { http_version } = self;
        Step2 {
            http_version,
            method,
        }
    }
}

#[derive(Debug)]
pub struct Step2 {
    http_version: Version,
    method: Method,
}

impl Step2 {
    pub fn path(self, path: UrlPath) -> Step3 {
        let Self {
            http_version,
            method,
        } = self;
        Step3 {
            http_version,
            method,
            path,
            query_string: QueryString::empty(),
            headers: Headers::default_for_request(),
        }
    }
}

#[derive(Debug)]
pub struct Step3 {
    http_version: Version,
    method: Method,
    path: UrlPath,
    query_string: QueryString,
    headers: Headers,
}

impl Step3 {
    pub fn add_qs_entry(self, qs_entry: QsEntry) -> Step3 {
        let Self {
            http_version,
            method,
            path,
            mut query_string,
            headers,
        } = self;

        query_string.add(qs_entry);

        Step3 {
            http_version,
            method,
            path,
            query_string,
            headers,
        }
    }
    pub fn add_header<H: IntoHeader>(self, header: H) -> Step3 {
        let Self {
            http_version,
            method,
            path,
            query_string,
            mut headers,
        } = self;

        headers.add(header.into_header());

        Step3 {
            http_version,
            method,
            path,
            query_string,
            headers,
        }
    }
    pub fn set_body(self, body: Body) -> Step4 {
        let Self {
            http_version,
            method,
            path,
            query_string,
            headers,
        } = self;
        Step4 {
            http_version,
            method,
            path,
            query_string,
            headers,
            body: Some(body),
        }
    }
    pub fn finish(self) -> Request {
        let Self {
            http_version,
            method,
            path,
            query_string,
            headers,
        } = self;

        Request {
            http_version,
            method,
            path,
            query_string,
            headers,
            body: None,
        }
    }
}

#[derive(Debug)]
pub struct Step4 {
    http_version: Version,
    method: Method,
    path: UrlPath,
    query_string: QueryString,
    headers: Headers,
    body: Option<Body>,
}
impl Step4 {
    pub fn finish(self) -> Request {
        let Self {
            http_version,
            method,
            path,
            query_string,
            headers,
            body,
        } = self;

        Request {
            http_version,
            method,
            path,
            query_string,
            headers,
            body,
        }
    }
}

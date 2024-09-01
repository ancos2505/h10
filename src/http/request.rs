pub mod builder;

#[cfg(test)]
mod tests;

use std::{fmt::Display, rc::Rc};

use builder::{
    RequestBuilderDelete, RequestBuilderGet, RequestBuilderHead, RequestBuilderLink,
    RequestBuilderPost, RequestBuilderPut, RequestBuilderUnlink,
};

use crate::{
    constants::{AsciiWhiteSpace, MAX_REQUEST_LENGTH},
    http::result::{H10LibError, H10LibResult},
};

use super::{
    body::Body,
    headers::Headers,
    method::Method,
    query_string::{QsEntry, QueryString},
    url_path::UrlPath,
    version::Version,
};

#[derive(Debug, Default)]
pub struct Request {
    pub http_version: Version,
    pub method: Method,
    pub path: UrlPath,
    pub query_string: QueryString,
    pub headers: Headers,
    pub body: Option<Body>,
}

impl Request {
    // * Builders
    pub fn delete() -> RequestBuilderDelete {
        RequestBuilderDelete
    }
    pub fn get() -> RequestBuilderGet {
        RequestBuilderGet
    }
    pub fn head() -> RequestBuilderHead {
        RequestBuilderHead
    }
    pub fn link() -> RequestBuilderLink {
        RequestBuilderLink
    }
    pub fn post() -> RequestBuilderPost {
        RequestBuilderPost
    }
    pub fn put() -> RequestBuilderPut {
        RequestBuilderPut
    }
    pub fn unlink() -> RequestBuilderUnlink {
        RequestBuilderUnlink
    }

    pub fn parse(bytes: &[u8]) -> H10LibResult<Request> {
        let now = std::time::Instant::now();
        if bytes.len() > MAX_REQUEST_LENGTH {
            return Err(H10LibError::RequestParser(format!(
                "Request size is larger than expected. MAX: {} Bytes",
                MAX_REQUEST_LENGTH
            )));
        }

        let headers_region = Self::get_header_region(bytes)?;

        let first_line_bytes = Self::get_method_line_bytes(headers_region)?;

        let method = Self::parse_method(first_line_bytes)?;

        let http_version = Self::parse_http_version(first_line_bytes)?;

        println!(
            "Security check proof in {} secs",
            now.elapsed().as_secs_f64()
        );

        let request_str = std::str::from_utf8(bytes)?;

        let rc_request_str: Rc<str> = request_str.into();

        let (headers_region, body_region) =
            rc_request_str
                .split_once("\r\n\r\n")
                .ok_or(H10LibError::RequestParser(
                    "Invalid HTTP Request on split headers and body".into(),
                ))?;

        let mut iter_headers = headers_region.split("\r\n");

        let first_line = iter_headers.next().ok_or(H10LibError::RequestParser(
            "Malformed HTTP Request Headers".into(),
        ))?;

        let maybe_headers_str = iter_headers.next();

        let (_, first_line_remaining) =
            first_line
                .split_once(AsciiWhiteSpace::as_str())
                .ok_or(H10LibError::RequestParser(
                    "Malformed HTTP Header Method line".into(),
                ))?;

        let (url_str, _) = first_line_remaining
            .split_once(AsciiWhiteSpace::as_str())
            .ok_or(H10LibError::RequestParser(
                "Malformed HTTP Header Method line on searching for Url path".into(),
            ))?;

        let (maybe_path_str, maybe_qs_str) = Self::parse_url(url_str)?;

        let path = match maybe_path_str {
            Some(inner_str) => UrlPath::parse(inner_str)?,
            None => UrlPath::root(),
        };

        let query_string = QueryString::parse(maybe_qs_str)?;

        let headers = Headers::parse(maybe_headers_str)?;

        let body = Some(body_region.parse()?);

        Ok(Request {
            method,
            http_version,
            path,
            query_string,
            headers,
            body,
        })
    }

    fn get_header_region<'a>(raw_request: &'a [u8]) -> H10LibResult<&'a [u8]> {
        let seq = b"\r\n\r\n";
        let mut i = 0;
        while i + seq.len() <= raw_request.len() {
            if raw_request[i..i + seq.len()] == *seq {
                return Ok(&raw_request[..(i + 2)]);
            }
            i += 1;
        }
        Err(H10LibError::RequestParser(
            "Invalid HTTP Request Header region".into(),
        ))
    }

    fn get_method_line_bytes<'a>(input: &'a [u8]) -> H10LibResult<&'a [u8]> {
        let seq = b"\r\n";
        let mut i = 0;
        while i + seq.len() <= input.len() {
            if input[i..i + seq.len()] == *seq {
                return Ok(&input[..i]);
            }
            i += 1;
        }
        Err(H10LibError::RequestParser(
            "Invalid HTTP Request header line".into(),
        ))
    }

    fn parse_method(input: &[u8]) -> H10LibResult<Method> {
        let bytes = input
            .split(|b| *b == b' ')
            .next()
            .ok_or(H10LibError::RequestParser(
                "Malformed bytes HTTP Method line on parsing Method".into(),
            ))?;

        if bytes.len() > Method::MAX_LENGTH {
            return Err(H10LibError::RequestParser(format!(
                "HTTP Method payload size is larger than expected. MAX: {} Bytes. Found: {} Bytes",
                Method::MAX_LENGTH,
                input.len()
            )));
        }

        let method_str = std::str::from_utf8(bytes)?;
        method_str.parse()
    }

    fn parse_http_version(input: &[u8]) -> H10LibResult<Version> {
        let mut iter = input.split(|b| *b == b' ');
        iter.next();
        iter.next();

        let bytes = iter.next().ok_or(H10LibError::RequestParser(
            "Malformed bytes HTTP Method lineon parsing HTTP Version".into(),
        ))?;

        if bytes.len() > Version::MAX_LENGTH {
            return Err(H10LibError::RequestParser(format!(
                "HTTP Version payload size is larger than expected. MAX: {} Bytes. Found: {} Bytes",
                Version::MAX_LENGTH,
                input.len()
            )));
        }

        let version = std::str::from_utf8(bytes)?;
        version.parse()
    }

    fn parse_url<'a>(input: &'a str) -> H10LibResult<(Option<&'a str>, Option<&'a str>)> {
        let trimmed = input.trim();
        if trimmed.contains("?") {
            let (path_str, query_string) = input.split_once("?").ok_or(
                H10LibError::RequestParser("Malformed UrlPath in HTTP Header Method line".into()),
            )?;
            Ok((Some(path_str), Some(query_string)))
        } else {
            Ok((Some(trimmed), None))
        }
    }

    pub fn http_version(&self) -> &Version {
        &self.http_version
    }

    pub fn method(&self) -> &Method {
        &self.method
    }

    pub fn path(&self) -> &UrlPath {
        &self.path
    }

    pub fn query_string(&self) -> &QueryString {
        &self.query_string
    }

    pub fn headers(&self) -> &Headers {
        &self.headers
    }

    pub fn body(&self) -> Option<&Body> {
        self.body.as_ref()
    }
}

impl Display for Request {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut output = "".to_owned();
        output.push_str(&self.method.to_string());
        output.push(' ');
        output.push_str(&self.path.to_string());
        output.push(' ');
        output.push_str(&self.http_version.to_string());
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

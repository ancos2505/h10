mod body;
pub mod old_parser;

#[cfg(test)]
mod tests;

use std::rc::Rc;

use crate::{
    constants::{AsciiWhiteSpace, MAX_REQUEST_LENGTH},
    http::result::{H10LibError, H10LibResult},
};

use super::{method::Method, version::Version};

#[derive(Debug, Default)]
pub struct Request {
    request_str: Rc<str>,
    pub http_version: Version,
    pub method: Method,
    pub path: Option<Rc<str>>,
    pub query: Vec<(Rc<str>, Rc<str>)>,
    headers: Vec<(Rc<str>, Rc<str>)>,
    body: Option<Rc<str>>,
}

impl Request {
    pub fn parse(bytes: &[u8]) -> H10LibResult<Request> {
        if bytes.len() > MAX_REQUEST_LENGTH {
            return Err(H10LibError::InvalidInputData(format!(
                "Request size is larger than expected. MAX: {} Bytes",
                MAX_REQUEST_LENGTH
            )));
        }

        let headers_region = Self::get_header_region(bytes)?;

        let first_line = Self::get_header_line(headers_region)?;

        let method = Self::parse_method(first_line)?;

        let http_version = Self::get_http_version(first_line)?;

        dbg!(&method, &http_version);

        // let method = Some(method_str.parse()?);

        let request_str = std::str::from_utf8(bytes)?;

        let rc_request_str: Rc<str> = request_str.into();

        let (headers_region, body_region) = rc_request_str
            .split_once("\r\n\r\n")
            .ok_or(H10LibError::RequestParser("Invalid HTTP Request".into()))?;

        let (first_line, headers_str) = headers_region
            .split_once("\r\n")
            .ok_or(H10LibError::RequestParser("Malformed HTTP Request".into()))?;

        let (_, remaining) =
            first_line
                .split_once(AsciiWhiteSpace::as_str())
                .ok_or(H10LibError::RequestParser(
                    "Malformed HTTP Method line".into(),
                ))?;

        let (path_str, version_str) =
            remaining
                .split_once(AsciiWhiteSpace::as_str())
                .ok_or(H10LibError::RequestParser(
                    "Malformed HTTP Method line".into(),
                ))?;

        let path = Some(Rc::from(path_str));

        Ok(Request {
            request_str: rc_request_str,
            method,
            http_version,
            path,
            query: Vec::new(),
            headers: Vec::new(),
            body: None,
        })
    }
    // TODO
    fn get_header_region<'a>(raw_request: &'a [u8]) -> H10LibResult<&'a [u8]> {
        let seq = b"\r\n\r\n";
        let mut i = 0;
        while i + seq.len() <= raw_request.len() {
            if raw_request[i..i + seq.len()] == *seq {
                return Ok(&raw_request[..i]);
            }
            i += 1;
        }
        Err(H10LibError::RequestParser("Invalid HTTP Request".into()))
    }
    fn get_header_line<'a>(input: &'a [u8]) -> H10LibResult<&'a [u8]> {
        let seq = b"\r\n";
        let mut i = 0;
        while i + seq.len() <= input.len() {
            if input[i..i + seq.len()] == *seq {
                return Ok(&input[..i]);
            }
            i += 1;
        }
        Err(H10LibError::RequestParser("Invalid HTTP Request".into()))
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
    fn get_http_version(input: &[u8]) -> H10LibResult<Version> {
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
}

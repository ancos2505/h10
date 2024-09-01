use core::str;
use std::rc::Rc;

use crate::{
    constants::{AsciiWhiteSpace, MAX_RESPONSE_LENGTH},
    http::{
        body::Body,
        headers::{ContentLength, IntoHeader},
        result::{H10LibError, H10LibResult},
        status_code::StatusCode,
    },
};

use crate::http::{headers::Headers, version::Version};

use super::Response;

#[derive(Debug)]
pub struct ResponseParser;

impl ResponseParser {
    pub fn parse(bytes: &[u8]) -> H10LibResult<Response> {
        let now = std::time::Instant::now();
        if bytes.len() > MAX_RESPONSE_LENGTH {
            return Err(H10LibError::ResponseParser(format!(
                "Response size is larger than expected. MAX: {} Bytes",
                MAX_RESPONSE_LENGTH
            )));
        }

        let headers_region = Self::get_header_region(bytes)?;

        let status_line_bytes = Self::get_status_line_bytes(headers_region)?;

        let http_version = Self::parse_http_version(status_line_bytes)?;

        let status = Self::parse_statuscode(status_line_bytes)?;

        println!(
            "Status-Line security check in {} secs",
            now.elapsed().as_secs_f64()
        );

        let valid_bytes = {
            let mut found = 0;
            for (idx, c) in bytes.iter().enumerate() {
                if *c == b'\0' {
                    found = idx;
                    break;
                }
            }
            let (valid, _) = bytes.split_at(found);
            valid
        };

        let request_str = std::str::from_utf8(valid_bytes)?;

        let rc_request_str: Rc<str> = request_str.into();

        // TODO: Check on RFC the expected behavior without body
        let (headers_region, body_region) =
            rc_request_str
                .split_once("\r\n\r\n")
                .ok_or(H10LibError::ResponseParser(
                    "Invalid HTTP Response on split headers and body".into(),
                ))?;

        // let first_line = headers_region
        //     .split("\r\n")
        //     .next()
        //     .ok_or(H10LibError::ResponseParser(
        //         "Malformed HTTP Response Headers".into(),
        //     ))?;

        // let (_, first_line_remaining) =
        //     first_line
        //         .split_once(AsciiWhiteSpace::as_str())
        //         .ok_or(H10LibError::ResponseParser(
        //             "Malformed HTTP Header Method line".into(),
        //         ))?;

        // let (url_str, _) = first_line_remaining
        //     .split_once(AsciiWhiteSpace::as_str())
        //     .ok_or(H10LibError::ResponseParser(
        //         "Malformed HTTP Header Method line on searching for Url path".into(),
        //     ))?;

        // let (maybe_path_str, maybe_qs_str) = Self::parse_url(url_str)?;

        let headers = Headers::parse(headers_region)?;

        let body: Body = body_region.parse()?;
        let body_length = body.len();

        let maybe_content_length = headers.get(ContentLength::default().into_header().name());
        if let Some(content_length) = maybe_content_length {
            if body_length != content_length.value().parse()? {
                return Err(H10LibError::InvalidInputData(
                    "Invalid body by Content-Length header".into(),
                ));
            }
        } else {
            if body_length > 0 {
                return Err(H10LibError::InvalidInputData(
                    "Invalid body by nonexistence of Content-Length header".into(),
                ));
            }
        }

        Ok(Response {
            http_version,
            status,
            headers,
            body: Some(body),
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
        Err(H10LibError::ResponseParser(
            "Invalid HTTP Response Header region".into(),
        ))
    }

    fn get_status_line_bytes<'a>(input: &'a [u8]) -> H10LibResult<&'a [u8]> {
        let seq = b"\r\n";
        let mut i = 0;
        while i + seq.len() <= input.len() {
            if input[i..i + seq.len()] == *seq {
                return Ok(&input[..i]);
            }
            i += 1;
        }
        Err(H10LibError::ResponseParser(
            "Invalid HTTP Response header line".into(),
        ))
    }

    fn parse_statuscode(input: &[u8]) -> H10LibResult<StatusCode> {
        use std::str;

        let mut iter = input.split(|b| *b == b' ');

        let _discard_version = iter.next();

        let bytes = iter.next().ok_or(H10LibError::ResponseParser(
            "Malformed bytes HTTP Method line on parsing Method".into(),
        ))?;

        if bytes.len() > StatusCode::MAX_LENGTH {
            return Err(H10LibError::ResponseParser(format!(
                "HTTP StatusCode payload size is larger than expected. MAX: {} Bytes. Found: {} Bytes",
                StatusCode::MAX_LENGTH,
                input.len()
            )));
        }

        let method_str = str::from_utf8(bytes)?;
        method_str.parse()
    }

    fn parse_http_version(input: &[u8]) -> H10LibResult<Version> {
        let bytes = input
            .split(|b| *b == b' ')
            .next()
            .ok_or(H10LibError::ResponseParser(
                "Malformed bytes HTTP Method lineon parsing HTTP Version".into(),
            ))?;

        if bytes.len() > Version::MAX_LENGTH {
            return Err(H10LibError::ResponseParser(format!(
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
                H10LibError::ResponseParser("Malformed UrlPath in HTTP Header Method line".into()),
            )?;
            Ok((Some(path_str), Some(query_string)))
        } else {
            Ok((Some(trimmed), None))
        }
    }
}

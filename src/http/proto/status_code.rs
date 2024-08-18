use std::{fmt::Display, marker::PhantomData};

#[derive(Debug, PartialEq, Eq)]
pub struct StatusCode<T: ValidCode> {
    pub number: usize,
    pub message: &'static str,
    _state: PhantomData<T>,
}

impl<T: ValidCode> StatusCode<T> {
    pub fn new() -> StatusCode<T> {
        StatusCode {
            number: T::number(),
            message: T::message(),
            _state: PhantomData,
        }
    }
}

impl<T: ValidCode> Display for StatusCode<T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut output = "".to_owned();
        output.push_str(&self.number.to_string());
        output.push(' ');
        output.push_str(&self.message.to_string());
        write!(f, "{}", output)
    }
}
pub trait ValidCode {
    fn number() -> usize;
    fn message() -> &'static str;
}

#[derive(Debug, PartialEq, Eq)]
pub struct Continue;

impl ValidCode for Continue {
    fn number() -> usize {
        100
    }
    fn message() -> &'static str {
        "Continue"
    }
}

impl StatusCode<Continue> {}

#[derive(Debug, PartialEq, Eq)]
pub struct SwitchingProtocols;

impl ValidCode for SwitchingProtocols {
    fn number() -> usize {
        101
    }
    fn message() -> &'static str {
        "Switching Protocols"
    }
}

impl StatusCode<SwitchingProtocols> {}

#[derive(Debug, PartialEq, Eq)]
pub struct OK;

impl ValidCode for OK {
    fn number() -> usize {
        200
    }
    fn message() -> &'static str {
        "OK"
    }
}
impl Display for OK {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", Self::message())
    }
}

impl StatusCode<OK> {}

#[derive(Debug, PartialEq, Eq)]
pub struct BadRequest;

impl ValidCode for BadRequest {
    fn number() -> usize {
        400
    }
    fn message() -> &'static str {
        "Bad Request"
    }
}

impl StatusCode<BadRequest> {}

#[derive(Debug, PartialEq, Eq)]
pub struct NotFound;

impl ValidCode for NotFound {
    fn number() -> usize {
        404
    }
    fn message() -> &'static str {
        "Not Found"
    }
}

impl StatusCode<NotFound> {}

#[derive(Debug, PartialEq, Eq)]
pub struct InternalServerError;

impl ValidCode for InternalServerError {
    fn number() -> usize {
        500
    }
    fn message() -> &'static str {
        "Internal Server Error"
    }
}

impl StatusCode<InternalServerError> {}

#[derive(Debug, PartialEq, Eq)]
pub struct NotImplemented;

impl ValidCode for NotImplemented {
    fn number() -> usize {
        501
    }
    fn message() -> &'static str {
        "Not Implemented"
    }
}

impl StatusCode<NotImplemented> {}

// 201 Created
// 202 Accepted
// 203 Non-Authoritative Information
// 204 No Content
// 205 Reset Content
// 206 Partial Content

// 300 Multiple Choices
// 301 Moved Permanently
// 302 Found
// 303 See Other
// 304 Not Modified
// 305 Use Proxy
// 306 (Unused)
// 307 Temporary Redirect
// 308 Permanent Redirect

// 401 Unauthorized
// 402 Payment Required
// 403 Forbidden
// 405 Method Not Allowed
// 406 Not Acceptable
// 407 Proxy Authentication Required
// 408 Request Timeout
// 409 Conflict
// 410 Gone
// 411 Length Required
// 412 Precondition Failed
// 413 Content Too Large
// 414 URI Too Long
// 415 Unsupported Media Type
// 416 Range Not Satisfiable
// 417 Expectation Failed
// 418 (Unused)
// 421 Misdirected Request
// 422 Unprocessable Content
// 426 Upgrade Required

// 502 Bad Gateway
// 503 Service Unavailable
// 504 Gateway Timeout
// 505 HTTP Version Not Supported

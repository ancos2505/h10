use std::fmt::{Debug, Display};

use crate::result::ServerResult;

pub trait IntoHeader {
    fn into_header(self) -> HttpHeader;
}

#[derive(Debug, PartialEq, Eq, Hash)]
pub struct HttpHeader {
    pub name: String,
    pub value: String,
}
impl Display for HttpHeader {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}: {}", self.name, self.value)
    }
}

#[derive(Debug, PartialEq, Eq)]
pub(crate) struct Server {
    name: String,
    value: String,
}
impl Default for Server {
    fn default() -> Self {
        Self {
            name: "Server".into(),
            value: format!(
                "{} (v{})",
                env!("CARGO_PKG_NAME"),
                env!("CARGO_PKG_VERSION")
            ),
        }
    }
}

impl Server {
    fn custom<S: ToString>(server_string: S) -> Self {
        Self {
            name: "Server".into(),
            value: server_string.to_string(),
        }
    }
}

impl IntoHeader for Server {
    fn into_header(self) -> HttpHeader {
        let Self { name, value } = self;
        HttpHeader { name, value }
    }
}

/// ### Date header
/// The unix epoch format it is intentional for standardized parsing between
/// devices especially embedded.
/// Reference: https://www.rfc-editor.org/rfc/rfc1945.html#section-10.6
#[derive(Debug, PartialEq, Eq)]
pub(crate) struct Date {
    name: String,
    value: String,
}
impl IntoHeader for Date {
    fn into_header(self) -> HttpHeader {
        let Self { name, value } = self;
        HttpHeader { name, value }
    }
}

impl Date {
    pub fn now() -> ServerResult<Self> {
        use std::time::{SystemTime, UNIX_EPOCH};
        let start = SystemTime::now();
        let since_the_epoch = start.duration_since(UNIX_EPOCH)?;
        let unix_epoch = since_the_epoch.as_secs();
        Ok(Self {
            name: "Date".into(),
            value: unix_epoch.to_string(),
        })
    }
}

/// ### Pragma - header
///
/// Reference: https://www.rfc-editor.org/rfc/rfc1945.html#section-10.12
#[derive(Debug, PartialEq, Eq)]
pub(crate) struct Pragma {
    name: String,
    value: String,
}
impl Default for Pragma {
    fn default() -> Self {
        Self {
            name: "Pragma".into(),
            value: "no-cache".into(),
        }
    }
}

impl IntoHeader for Pragma {
    fn into_header(self) -> HttpHeader {
        let Self { name, value } = self;
        HttpHeader { name, value }
    }
}

#[derive(Debug, PartialEq, Eq)]
pub(crate) struct Connection {
    name: String,
    value: String,
}

impl IntoHeader for Connection {
    fn into_header(self) -> HttpHeader {
        let Self { name, value } = self;
        HttpHeader { name, value }
    }
}

impl Connection {
    /// This is the default on HTTP/1.0 requests.
    pub fn close() -> Self {
        Self {
            name: "Connection".into(),
            value: "close".into(),
        }
    }
    /// This is the default on HTTP/1.1 requests.
    pub fn keep_alive() -> Self {
        Self {
            name: "Connection".into(),
            value: "keep-alive".into(),
        }
    }
}

#[derive(Debug, PartialEq, Eq)]
pub(crate) struct ContentType {
    name: String,
    value: String,
}

impl ContentType {
    pub fn html() -> Self {
        Self {
            name: "Content-Type".into(),
            value: "text/html; charset=UTF-8".into(),
        }
    }
    pub fn javascript() -> Self {
        Self {
            name: "Content-Type".into(),
            value: "application/javascript; charset=UTF-8".into(),
        }
    }
    pub fn json() -> Self {
        Self {
            name: "Content-Type".into(),
            value: "application/json; charset=UTF-8".into(),
        }
    }
}

impl IntoHeader for ContentType {
    fn into_header(self) -> HttpHeader {
        let Self { name, value } = self;
        HttpHeader { name, value }
    }
}

#[derive(Debug, PartialEq, Eq)]
pub(crate) struct ContentLength {
    name: String,
    value: usize,
}
impl ContentLength {
    pub fn length(len: usize) -> Self {
        Self {
            name: "Content-Length".into(),
            value: len,
        }
    }
}
impl IntoHeader for ContentLength {
    fn into_header(self) -> HttpHeader {
        let Self { name, value } = self;
        HttpHeader {
            name,
            value: value.to_string(),
        }
    }
}

// Request Header Fields
//
// Reference: https://www.rfc-editor.org/rfc/rfc1945.html#section-5.2

/// ### Authorization
/// Related: Authentication
/// Reference: https://www.rfc-editor.org/rfc/rfc1945.html#section-10.2
#[derive(Debug, PartialEq, Eq)]
pub(crate) struct Authorization {
    name: String,
    value: String,
}

/// ### From
/// Related: Public Key Infrastructure
/// Reference: https://www.rfc-editor.org/rfc/rfc1945.html#section-10.8
#[derive(Debug, PartialEq, Eq)]
pub(crate) struct From {
    name: String,
    value: String,
}
/// ### If-Modified-Since
/// Related:  304 (not modified) response will be returned without any
///          Entity-Body.
/// Reference: https://www.rfc-editor.org/rfc/rfc1945.html#section-10.9
#[derive(Debug, PartialEq, Eq)]
pub(crate) struct IfModifiedSince {
    name: String,
    value: String,
}

/// ### Referer
/// Related:  back-links to resources for interest, logging, optimized caching,
///          etc.
/// Reference: https://www.rfc-editor.org/rfc/rfc1945.html#section-10.13
#[derive(Debug, PartialEq, Eq)]
pub(crate) struct Referer {
    name: String,
    value: String,
}

/// ### User-Agent
/// Related: Browser/2.14 Library/3.57
/// Reference: https://www.rfc-editor.org/rfc/rfc1945.html#section-10.15
#[derive(Debug, PartialEq, Eq)]
pub(crate) struct UserAgent {
    name: String,
    value: String,
}

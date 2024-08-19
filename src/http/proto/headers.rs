use std::fmt::{Debug, Display};

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
pub(crate) struct Host(String);

impl From<Host> for (String, String) {
    fn from(value: Host) -> Self {
        ("Host".into(), value.0)
    }
}

#[derive(Debug, PartialEq, Eq)]
pub(crate) struct UserAgent(String);

impl From<UserAgent> for (String, String) {
    fn from(value: UserAgent) -> Self {
        ("User-Agent".into(), value.0)
    }
}

#[derive(Debug, PartialEq, Eq)]
pub(crate) struct Accept(String);

impl From<Accept> for (String, String) {
    fn from(value: Accept) -> Self {
        ("Accept".into(), value.0)
    }
}

#[derive(Debug, PartialEq, Eq)]
pub(crate) struct Server {
    name: String,
    value: String,
}

#[derive(Debug, PartialEq, Eq)]
pub(crate) struct Date {
    name: String,
    value: String,
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

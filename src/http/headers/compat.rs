use std::str::FromStr;

use crate::http::{
    headers::{HeaderEntry, HeaderName, HeaderValue, IntoHeader},
    result::H10LibError,
};

/// ### Connection
/// Related: HTTP/1.1 compatibility
///
///  "Persistent connections are the default for HTTP/1.1 messages; we introduce
/// a new keyword (Connection: close) for declaring non-persistence." -
/// **RFC2068#section-19.7.1**
///
/// References:
/// - https://www.rfc-editor.org/rfc/rfc2068#section-14.10
/// - https://www.rfc-editor.org/rfc/rfc2068#section-19.7.1
///
#[derive(Debug, PartialEq, Eq)]
pub struct Connection {
    name: HeaderName,
    value: HeaderValue,
}

impl Default for Connection {
    fn default() -> Self {
        Self {
            name: HeaderName::new_unchecked("Connection"),
            value: HeaderValue::new_unchecked("close"),
        }
    }
}

impl FromStr for Connection {
    type Err = H10LibError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let entry: HeaderEntry = s.parse()?;
        Ok(entry.into())
    }
}

impl From<HeaderEntry> for Connection {
    fn from(value: HeaderEntry) -> Self {
        let HeaderEntry { name, value } = value;
        Self { name, value }
    }
}

impl IntoHeader for Connection {
    fn into_header(self) -> HeaderEntry {
        let Self { name, value } = self;
        HeaderEntry { name, value }
    }
}

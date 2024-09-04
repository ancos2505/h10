use std::str::FromStr;

use crate::http::{
    headers::{HeaderEntry, HeaderName, HeaderValue, IntoHeader},
    result::H10LibError,
};

/// ### MIME-Version
/// Related: Content handling
///
///  HTTP messages may include a single MIME-Version general-header field to
/// indicate what version of the MIME protocol was used to construct the
/// message. Use of the MIME-Version header field, as defined by RFC 1521 [5],
/// should indicate that the message is MIME-conformant. Unfortunately, some
/// older HTTP/1.0 servers send it indiscriminately, and thus this field should
/// be ignored.
///
/// Reference: https://www.rfc-editor.org/rfc/rfc1945.html#appendix-D.2.7
///
#[derive(Debug, PartialEq, Eq)]
pub struct MIMEVersion {
    name: HeaderName,
    value: HeaderValue,
}

impl Default for MIMEVersion {
    fn default() -> Self {
        Self {
            name: HeaderName::new_unchecked("MIME-Version"),
            value: HeaderValue::new_unchecked("Not_Defined"),
        }
    }
}

impl FromStr for MIMEVersion {
    type Err = H10LibError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let entry: HeaderEntry = s.parse()?;
        Ok(entry.into())
    }
}

impl From<HeaderEntry> for MIMEVersion {
    fn from(value: HeaderEntry) -> Self {
        let HeaderEntry { name, value } = value;
        Self { name, value }
    }
}

impl IntoHeader for MIMEVersion {
    fn into_header(self) -> HeaderEntry {
        let Self { name, value } = self;
        HeaderEntry { name, value }
    }
}

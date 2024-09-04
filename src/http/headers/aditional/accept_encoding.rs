use std::str::FromStr;

use crate::http::{
    headers::{HeaderEntry, HeaderName, HeaderValue, IntoHeader},
    result::H10LibError,
};

/// ### Accept-Encoding
/// Related: Content handling
///
///  The Accept-Encoding request-header field is similar to Accept, but
/// restricts the content-coding values which are acceptable in the response.
///
/// Reference: https://www.rfc-editor.org/rfc/rfc1945.html#appendix-D.2.3
///
#[derive(Debug, PartialEq, Eq)]
pub struct AcceptEncoding {
    name: HeaderName,
    value: HeaderValue,
}

impl Default for AcceptEncoding {
    fn default() -> Self {
        Self {
            name: HeaderName::new_unchecked("Accept-Encoding"),
            value: HeaderValue::new_unchecked("Not_Defined"),
        }
    }
}

impl FromStr for AcceptEncoding {
    type Err = H10LibError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let entry: HeaderEntry = s.parse()?;
        Ok(entry.into())
    }
}

impl From<HeaderEntry> for AcceptEncoding {
    fn from(value: HeaderEntry) -> Self {
        let HeaderEntry { name, value } = value;
        Self { name, value }
    }
}

impl IntoHeader for AcceptEncoding {
    fn into_header(self) -> HeaderEntry {
        let Self { name, value } = self;
        HeaderEntry { name, value }
    }
}

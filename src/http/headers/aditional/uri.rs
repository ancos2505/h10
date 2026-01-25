use std::str::FromStr;

use crate::http::{
    headers::{HeaderEntry, HeaderName, HeaderValue, IntoHeader},
    result::H10LibError,
};

/// ### URI
/// Related: Content handling
///
///  The URI entity-header field may contain some or all of the Uniform Resource
/// Identifiers (Section 3.2) by which the Request-URI resource can be
/// identified. There is no guarantee that the resource can be accessed using
/// the URI(s) specified.
///
/// Reference: https://www.rfc-editor.org/rfc/rfc1945.html#appendix-D.2.6
///
#[derive(Debug, PartialEq, Eq)]
pub struct URI {
    name: HeaderName,
    value: HeaderValue,
}

impl Default for URI {
    fn default() -> Self {
        Self {
            name: HeaderName::new_unchecked("URI"),
            value: HeaderValue::new_unchecked("Not_Defined"),
        }
    }
}

impl FromStr for URI {
    type Err = H10LibError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let entry: HeaderEntry = s.parse()?;
        Ok(entry.into())
    }
}

impl From<HeaderEntry> for URI {
    fn from(value: HeaderEntry) -> Self {
        let HeaderEntry { name, value } = value;
        Self { name, value }
    }
}

impl IntoHeader for URI {
    fn into_header(self) -> HeaderEntry {
        let Self { name, value } = self;
        HeaderEntry { name, value }
    }
}

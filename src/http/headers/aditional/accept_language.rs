use std::str::FromStr;

use crate::http::{
    headers::{HeaderEntry, HeaderName, HeaderValue, IntoHeader},
    result::H10LibError,
};

/// ### Accept-Language
/// Related: Content handling
///
///  The Accept-Language request-header field is similar to Accept, but
/// restricts the set of natural languages that are preferred as a response to
/// the request.
///
/// Reference: https://www.rfc-editor.org/rfc/rfc1945.html#appendix-D.2.4
///
#[derive(Debug, PartialEq, Eq)]
pub struct AcceptLanguage {
    name: HeaderName,
    value: HeaderValue,
}

impl Default for AcceptLanguage {
    fn default() -> Self {
        Self {
            name: HeaderName::new_unchecked("Accept-Language"),
            value: HeaderValue::new_unchecked("Not_Defined"),
        }
    }
}

impl FromStr for AcceptLanguage {
    type Err = H10LibError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let entry: HeaderEntry = s.parse()?;
        Ok(entry.into())
    }
}

impl From<HeaderEntry> for AcceptLanguage {
    fn from(value: HeaderEntry) -> Self {
        let HeaderEntry { name, value } = value;
        Self { name, value }
    }
}

impl IntoHeader for AcceptLanguage {
    fn into_header(self) -> HeaderEntry {
        let Self { name, value } = self;
        HeaderEntry { name, value }
    }
}

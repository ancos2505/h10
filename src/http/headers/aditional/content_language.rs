use std::str::FromStr;

use crate::http::{
    headers::{HeaderEntry, HeaderName, HeaderValue, IntoHeader},
    result::H10LibError,
};

/// ### Content-Language
/// Related: Content handling
///
///  The Content-Language entity-header field describes the natural language(s)
/// of the intended audience for the enclosed entity. Note that this may not be
/// equivalent to all the languages used within the entity.
///
/// Reference: https://www.rfc-editor.org/rfc/rfc1945.html#appendix-D.2.5
///
#[derive(Debug, PartialEq, Eq)]
pub struct ContentLanguage {
    name: HeaderName,
    value: HeaderValue,
}

impl Default for ContentLanguage {
    fn default() -> Self {
        Self {
            name: HeaderName::new_unchecked("Content-Language"),
            value: HeaderValue::new_unchecked("Not_Defined"),
        }
    }
}

impl FromStr for ContentLanguage {
    type Err = H10LibError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let entry: HeaderEntry = s.parse()?;
        Ok(entry.into())
    }
}

impl From<HeaderEntry> for ContentLanguage {
    fn from(value: HeaderEntry) -> Self {
        let HeaderEntry { name, value } = value;
        Self { name, value }
    }
}

impl IntoHeader for ContentLanguage {
    fn into_header(self) -> HeaderEntry {
        let Self { name, value } = self;
        HeaderEntry { name, value }
    }
}

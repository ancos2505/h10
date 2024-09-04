use std::str::FromStr;

use crate::http::{
    headers::{HeaderEntry, HeaderName, HeaderValue, IntoHeader},
    result::H10LibError,
};

/// ### Link
/// Related: Content handling
///
///  The Link entity-header field provides a means for describing a relationship
/// between the entity and some other resource. An entity may include multiple
/// Link values. Links at the metainformation level typically indicate
/// relationships like hierarchical structure and navigation paths.
///
/// Reference: https://www.rfc-editor.org/rfc/rfc1945.html#appendix-D.2.6
///
#[derive(Debug, PartialEq, Eq)]
pub struct Link {
    name: HeaderName,
    value: HeaderValue,
}

impl Default for Link {
    fn default() -> Self {
        Self {
            name: HeaderName::new_unchecked("Link"),
            value: HeaderValue::new_unchecked("Not_Defined"),
        }
    }
}

impl FromStr for Link {
    type Err = H10LibError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let entry: HeaderEntry = s.parse()?;
        Ok(entry.into())
    }
}

impl From<HeaderEntry> for Link {
    fn from(value: HeaderEntry) -> Self {
        let HeaderEntry { name, value } = value;
        Self { name, value }
    }
}

impl IntoHeader for Link {
    fn into_header(self) -> HeaderEntry {
        let Self { name, value } = self;
        HeaderEntry { name, value }
    }
}

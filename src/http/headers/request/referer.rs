use std::str::FromStr;

use crate::http::{
    headers::{HeaderEntry, HeaderName, HeaderValue, IntoHeader},
    result::H10LibError,
};

/// ### Referer
/// Related:  back-links to resources for interest, logging, optimized caching,
///          etc.
///
/// Reference: https://www.rfc-editor.org/rfc/rfc1945.html#section-10.13
#[derive(Debug, PartialEq, Eq)]
pub struct Referer {
    name: HeaderName,
    value: HeaderValue,
}

impl Default for Referer {
    fn default() -> Self {
        Self {
            name: HeaderName::new_unchecked("Referer"),
            value: HeaderValue::new_unchecked("Not_Defined"),
        }
    }
}

impl FromStr for Referer {
    type Err = H10LibError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let entry: HeaderEntry = s.parse()?;
        Ok(entry.into())
    }
}

impl From<HeaderEntry> for Referer {
    fn from(value: HeaderEntry) -> Self {
        let HeaderEntry { name, value } = value;
        Self { name, value }
    }
}

impl IntoHeader for Referer {
    fn into_header(self) -> HeaderEntry {
        let Self { name, value } = self;
        HeaderEntry { name, value }
    }
}

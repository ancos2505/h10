use std::str::FromStr;

use crate::http::{
    headers::{HeaderEntry, HeaderName, HeaderValue, IntoHeader},
    result::H10LibError,
};

/// ### Allow
/// Related:  It is strictly to inform the recipient of valid methods associated
///          with the resource.
///
/// Reference: https://www.rfc-editor.org/rfc/rfc1945.html#section-10.1
#[derive(Debug, PartialEq, Eq)]
pub struct Allow {
    name: HeaderName,
    value: HeaderValue,
}

impl Default for Allow {
    fn default() -> Self {
        Self {
            name: HeaderName::new_unchecked("Allow"),
            value: HeaderValue::new_unchecked("Not_Defined"),
        }
    }
}

impl FromStr for Allow {
    type Err = H10LibError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let entry: HeaderEntry = s.parse()?;
        Ok(entry.into())
    }
}

impl From<HeaderEntry> for Allow {
    fn from(value: HeaderEntry) -> Self {
        let HeaderEntry { name, value } = value;
        Self { name, value }
    }
}

impl IntoHeader for Allow {
    fn into_header(self) -> HeaderEntry {
        let Self { name, value } = self;
        HeaderEntry { name, value }
    }
}

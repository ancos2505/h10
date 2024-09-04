use std::str::FromStr;

use crate::http::{
    headers::{HeaderEntry, HeaderName, HeaderValue, IntoHeader},
    result::H10LibError,
};

/// ### Pragma - header
/// Related: Content state
///
/// Reference: https://www.rfc-editor.org/rfc/rfc1945.html#section-10.12
#[derive(Debug, PartialEq, Eq)]
pub struct Pragma {
    name: HeaderName,
    value: HeaderValue,
}
impl Default for Pragma {
    fn default() -> Self {
        Self {
            name: HeaderName::new_unchecked("Pragma"),
            value: HeaderValue::new_unchecked("no-cache"),
        }
    }
}

impl FromStr for Pragma {
    type Err = H10LibError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let entry: HeaderEntry = s.parse()?;
        Ok(entry.into())
    }
}

impl From<HeaderEntry> for Pragma {
    fn from(value: HeaderEntry) -> Self {
        let HeaderEntry { name, value } = value;
        Self { name, value }
    }
}

impl IntoHeader for Pragma {
    fn into_header(self) -> HeaderEntry {
        let Self { name, value } = self;
        HeaderEntry { name, value }
    }
}

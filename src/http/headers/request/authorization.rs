use std::str::FromStr;

use crate::http::{
    headers::{HeaderEntry, HeaderName, HeaderValue, IntoHeader},
    result::{H10LibError, H10LibResult},
};

/// ### Authorization
/// Related: Authentication
///
/// Reference: https://www.rfc-editor.org/rfc/rfc1945.html#section-10.2
#[derive(Debug, PartialEq, Eq)]
pub struct Authorization {
    name: HeaderName,
    value: HeaderValue,
}

impl Default for Authorization {
    fn default() -> Self {
        Self {
            name: HeaderName::new_unchecked("Authorization"),
            value: HeaderValue::new_unchecked("Not_Defined"),
        }
    }
}

impl Authorization {
    pub fn new(new_value: &str) -> H10LibResult<Self> {
        Ok(Self {
            value: new_value.parse()?,
            ..Default::default()
        })
    }
}

impl FromStr for Authorization {
    type Err = H10LibError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let entry: HeaderEntry = s.parse()?;
        Ok(entry.into())
    }
}

impl From<HeaderEntry> for Authorization {
    fn from(value: HeaderEntry) -> Self {
        let HeaderEntry { name, value } = value;
        Self { name, value }
    }
}

impl IntoHeader for Authorization {
    fn into_header(self) -> HeaderEntry {
        let Self { name, value } = self;
        HeaderEntry { name, value }
    }
}

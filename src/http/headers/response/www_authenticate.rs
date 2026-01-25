use std::str::FromStr;

use crate::http::{
    headers::{HeaderEntry, HeaderName, HeaderValue, IntoHeader},
    result::{H10LibError, H10LibResult},
};

/// ### WWW-Authenticate header
/// Related: Authentication/Authorization/Session
///
/// Reference: https://www.rfc-editor.org/rfc/rfc1945.html#section-10.16
#[derive(Debug, PartialEq, Eq)]
pub struct WWWAuthenticate {
    name: HeaderName,
    value: HeaderValue,
}

impl Default for WWWAuthenticate {
    fn default() -> Self {
        Self {
            name: HeaderName::new_unchecked("WWW-Authenticate"),
            value: HeaderValue::new_unchecked("Not_Defined"),
        }
    }
}

impl WWWAuthenticate {
    pub fn new(new_value: &str) -> H10LibResult<Self> {
        Ok(Self {
            value: new_value.parse()?,
            ..Default::default()
        })
    }
}

impl FromStr for WWWAuthenticate {
    type Err = H10LibError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let entry: HeaderEntry = s.parse()?;
        Ok(entry.into())
    }
}

impl From<HeaderEntry> for WWWAuthenticate {
    fn from(value: HeaderEntry) -> Self {
        let HeaderEntry { name, value } = value;
        Self { name, value }
    }
}

impl IntoHeader for WWWAuthenticate {
    fn into_header(self) -> HeaderEntry {
        let Self { name, value } = self;
        HeaderEntry { name, value }
    }
}

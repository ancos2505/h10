use std::str::FromStr;

use crate::http::{
    headers::{HeaderEntry, HeaderName, HeaderValue, IntoHeader},
    result::{H10LibError, H10LibResult},
};

/// ### Date header
/// The unix epoch format it is intentional for standardized parsing between
/// devices especially embedded.
/// Reference: https://www.rfc-editor.org/rfc/rfc1945.html#section-10.6
#[derive(Debug, PartialEq, Eq)]
pub struct Date {
    name: HeaderName,
    value: HeaderValue,
}

impl Default for Date {
    fn default() -> Self {
        Self {
            name: HeaderName::new_unchecked("Date"),
            value: HeaderValue::new_unchecked("Not_Defined"),
        }
    }
}
impl FromStr for Date {
    type Err = H10LibError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let entry: HeaderEntry = s.parse()?;
        Ok(entry.into())
    }
}

impl From<HeaderEntry> for Date {
    fn from(value: HeaderEntry) -> Self {
        let HeaderEntry { name, value } = value;
        Self { name, value }
    }
}

impl IntoHeader for Date {
    fn into_header(self) -> HeaderEntry {
        let Self { name, value } = self;
        HeaderEntry { name, value }
    }
}

impl Date {
    pub fn now() -> H10LibResult<Self> {
        use std::time::{SystemTime, UNIX_EPOCH};
        let start = SystemTime::now();
        let since_the_epoch = start.duration_since(UNIX_EPOCH)?;
        let unix_epoch = since_the_epoch.as_secs();
        Ok(Self {
            value: unix_epoch.to_string().parse()?,
            ..Default::default()
        })
    }
}

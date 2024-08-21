use crate::http::{
    headers::{HttpHeader, IntoHeader},
    result::H10LibResult,
};

/// ### Date header
/// The unix epoch format it is intentional for standardized parsing between
/// devices especially embedded.
/// Reference: https://www.rfc-editor.org/rfc/rfc1945.html#section-10.6
#[derive(Debug, PartialEq, Eq)]
pub struct Date {
    name: String,
    value: String,
}
impl IntoHeader for Date {
    fn into_header(self) -> HttpHeader {
        let Self { name, value } = self;
        HttpHeader { name, value }
    }
}

impl Date {
    pub fn now() -> H10LibResult<Self> {
        use std::time::{SystemTime, UNIX_EPOCH};
        let start = SystemTime::now();
        let since_the_epoch = start.duration_since(UNIX_EPOCH)?;
        let unix_epoch = since_the_epoch.as_secs();
        Ok(Self {
            name: "Date".into(),
            value: unix_epoch.to_string(),
        })
    }
}

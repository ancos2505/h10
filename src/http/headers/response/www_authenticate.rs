use crate::http::headers::{HeaderEntry, HeaderName, HeaderValue, IntoHeader};

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

impl IntoHeader for WWWAuthenticate {
    fn into_header(self) -> HeaderEntry {
        let Self { name, value } = self;
        HeaderEntry { name, value }
    }
}

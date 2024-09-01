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

// impl WWWAuthenticate {
//     fn example() -> Self {
//         Self {
//             name: HeaderName::new_unchecked("WWW-Authenticate"),
//             value: HeaderValue::new_unchecked("NotDefined"),
//         }
//     }
// }

impl IntoHeader for WWWAuthenticate {
    fn into_header(self) -> HeaderEntry {
        let Self { name, value } = self;
        HeaderEntry { name, value }
    }
}

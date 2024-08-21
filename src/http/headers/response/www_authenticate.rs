use crate::http::headers::{HttpHeader, IntoHeader};

/// ### WWW-Authenticate header
/// Related: Authentication/Authorization/Session
///
/// Reference: https://www.rfc-editor.org/rfc/rfc1945.html#section-10.16
#[derive(Debug, PartialEq, Eq)]
pub struct WWWAuthenticate {
    name: String,
    value: String,
}
impl Default for WWWAuthenticate {
    fn default() -> Self {
        Self {
            name: "WWW-Authenticate".into(),
            value: format!(
                "{} (v{})",
                env!("CARGO_PKG_NAME"),
                env!("CARGO_PKG_VERSION")
            ),
        }
    }
}

impl IntoHeader for WWWAuthenticate {
    fn into_header(self) -> HttpHeader {
        let Self { name, value } = self;
        HttpHeader { name, value }
    }
}

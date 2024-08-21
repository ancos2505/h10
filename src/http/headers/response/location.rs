use crate::http::headers::{HttpHeader, IntoHeader};

/// ### Location header
/// Related: Indicate a redirection
///
/// Reference: https://www.rfc-editor.org/rfc/rfc1945.html#section-10.11
#[derive(Debug, PartialEq, Eq)]
pub struct Location {
    name: String,
    value: String,
}
impl Default for Location {
    fn default() -> Self {
        Self {
            name: "Location".into(),
            value: format!(
                "{} (v{})",
                env!("CARGO_PKG_NAME"),
                env!("CARGO_PKG_VERSION")
            ),
        }
    }
}

impl IntoHeader for Location {
    fn into_header(self) -> HttpHeader {
        let Self { name, value } = self;
        HttpHeader { name, value }
    }
}

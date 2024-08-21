use crate::http::proto::headers::{HttpHeader, IntoHeader};

/// ### Pragma - header
/// Related: Content state
///
/// Reference: https://www.rfc-editor.org/rfc/rfc1945.html#section-10.12
#[derive(Debug, PartialEq, Eq)]
pub(crate) struct Pragma {
    name: String,
    value: String,
}
impl Default for Pragma {
    fn default() -> Self {
        Self {
            name: "Pragma".into(),
            value: "no-cache".into(),
        }
    }
}

impl IntoHeader for Pragma {
    fn into_header(self) -> HttpHeader {
        let Self { name, value } = self;
        HttpHeader { name, value }
    }
}

use crate::http::proto::headers::{HttpHeader, IntoHeader};

/// ### Server header
/// Related: Server/2.14 Library/3.57
/// 
/// Reference: https://www.rfc-editor.org/rfc/rfc1945.html#section-10.14
#[derive(Debug, PartialEq, Eq)]
pub(crate) struct Server {
    name: String,
    value: String,
}
impl Default for Server {
    fn default() -> Self {
        Self {
            name: "Server".into(),
            value: format!(
                "{} (v{})",
                env!("CARGO_PKG_NAME"),
                env!("CARGO_PKG_VERSION")
            ),
        }
    }
}

impl Server {
    pub(crate) fn custom<S: ToString>(server_string: S) -> Self {
        Self {
            name: "Server".into(),
            value: server_string.to_string(),
        }
    }
}

impl IntoHeader for Server {
    fn into_header(self) -> HttpHeader {
        let Self { name, value } = self;
        HttpHeader { name, value }
    }
}

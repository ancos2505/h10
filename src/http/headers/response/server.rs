use crate::http::{
    headers::{HeaderEntry, HeaderName, HeaderValue, IntoHeader},
    result::H10LibResult,
};

/// ### Server header
/// Related: Server/2.14 Library/3.57
///
/// Reference: https://www.rfc-editor.org/rfc/rfc1945.html#section-10.14
#[derive(Debug, PartialEq, Eq)]
pub struct Server {
    name: HeaderName,
    value: HeaderValue,
}

impl Default for Server {
    fn default() -> Self {
        let value = format!(
            "{} (v{})",
            env!("CARGO_PKG_NAME"),
            env!("CARGO_PKG_VERSION")
        );

        Self {
            name: HeaderName::new_unchecked("Server"),
            value: HeaderValue::new_unchecked(&value),
        }
    }
}

impl Server {
    #[allow(dead_code)]
    pub(crate) fn custom<S: AsRef<str>>(server_string: S) -> H10LibResult<Self> {
        let value_str = server_string.as_ref();

        Ok(Self {
            value: value_str.parse()?,
            ..Default::default()
        })
    }
}

impl IntoHeader for Server {
    fn into_header(self) -> HeaderEntry {
        let Self { name, value } = self;
        HeaderEntry { name, value }
    }
}

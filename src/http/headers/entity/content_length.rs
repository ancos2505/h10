use crate::http::headers::{HttpHeader, IntoHeader};

/// ### Content-Length header
/// Related: Entity-Body
///
///  The Content-Length entity-header field indicates the size of the
/// Entity-Body, in decimal number of octets, sent to the recipient or, in the
/// case of the HEAD method, the size of the Entity-Body that would have been
/// sent had the request been a GET.
///
/// **Reference:** https://www.rfc-editor.org/rfc/rfc1945.html#section-10.4
///
#[derive(Debug, PartialEq, Eq)]
pub struct ContentLength {
    name: String,
    value: usize,
}
impl ContentLength {
    pub fn length(len: usize) -> Self {
        Self {
            name: "Content-Length".into(),
            value: len,
        }
    }
}
impl IntoHeader for ContentLength {
    fn into_header(self) -> HttpHeader {
        let Self { name, value } = self;
        HttpHeader {
            name,
            value: value.to_string(),
        }
    }
}

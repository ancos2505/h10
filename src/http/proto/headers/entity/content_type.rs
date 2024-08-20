use crate::http::proto::headers::{HttpHeader, IntoHeader};

/// ### Content-Type header
/// Related: Entity-Body
///
///  The Content-Type entity-header field indicates the media type of the
/// Entity-Body sent to the recipient or, in the case of the HEAD method, the
/// media type that would have been sent had the request been a GET.
///
///  If the media type remains unknown, the recipient should treat it as type
/// "application/octet-stream".
///
/// **Reference:** https://www.rfc-editor.org/rfc/rfc1945.html#section-10.5
///
#[derive(Debug, PartialEq, Eq)]
pub(crate) struct ContentType {
    name: String,
    value: String,
}

impl ContentType {
    pub fn octet_stream() -> Self {
        Self {
            name: "Content-Type".into(),
            value: "application/octet-stream".into(),
        }
    }
    pub fn html() -> Self {
        Self {
            name: "Content-Type".into(),
            value: "text/html; charset=UTF-8".into(),
        }
    }
    pub fn javascript() -> Self {
        Self {
            name: "Content-Type".into(),
            value: "application/javascript; charset=UTF-8".into(),
        }
    }
    pub fn json() -> Self {
        Self {
            name: "Content-Type".into(),
            value: "application/json; charset=UTF-8".into(),
        }
    }
}

impl IntoHeader for ContentType {
    fn into_header(self) -> HttpHeader {
        let Self { name, value } = self;
        HttpHeader { name, value }
    }
}

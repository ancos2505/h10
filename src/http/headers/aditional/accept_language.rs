use crate::http::headers::{HeaderEntry, HeaderName, HeaderValue, IntoHeader};

/// ### Accept-Language
/// Related: Content handling
///
///  The Accept-Language request-header field is similar to Accept, but
/// restricts the set of natural languages that are preferred as a response to
/// the request.
///
/// Reference: https://www.rfc-editor.org/rfc/rfc1945.html#appendix-D.2.4
///
#[derive(Debug, PartialEq, Eq)]
pub struct AcceptLanguage {
    name: HeaderName,
    value: HeaderValue,
}

impl IntoHeader for AcceptLanguage {
    fn into_header(self) -> HeaderEntry {
        let Self { name, value } = self;
        HeaderEntry { name, value }
    }
}

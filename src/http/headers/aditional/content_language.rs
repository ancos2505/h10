use crate::http::headers::{HeaderEntry, HeaderName, HeaderValue, IntoHeader};

/// ### Content-Language
/// Related: Content handling
///
///  The Content-Language entity-header field describes the natural language(s)
/// of the intended audience for the enclosed entity. Note that this may not be
/// equivalent to all the languages used within the entity.
///
/// Reference: https://www.rfc-editor.org/rfc/rfc1945.html#appendix-D.2.5
///
#[derive(Debug)]
pub struct ContentLanguage {
    name: HeaderName,
    value: HeaderValue,
}

impl IntoHeader for ContentLanguage {
    fn into_header(self) -> HeaderEntry {
        let Self { name, value } = self;
        HeaderEntry { name, value }
    }
}

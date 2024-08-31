use crate::http::headers::{HeaderName, HeaderValue};

/// ### Authorization
/// Related: Authentication
///
/// Reference: https://www.rfc-editor.org/rfc/rfc1945.html#section-10.2
#[derive(Debug, PartialEq, Eq)]
pub struct Authorization {
    name: HeaderName,
    value: HeaderValue,
}

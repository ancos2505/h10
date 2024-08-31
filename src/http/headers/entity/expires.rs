use crate::http::headers::{HeaderName, HeaderValue};

/// ### Expires
/// Related: Resource state
///
///  The Expires entity-header field gives the date/time after which the entity
/// should be considered stale.
///
/// Reference: https://www.rfc-editor.org/rfc/rfc1945.html#section-10.7
///
#[derive(Debug, PartialEq, Eq)]
pub struct Expires {
    name: HeaderName,
    value: HeaderValue,
}

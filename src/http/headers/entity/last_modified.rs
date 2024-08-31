use crate::http::headers::{HeaderName, HeaderValue};

/// ### Last-Modified
/// Related: Content state
///
///  The Last-Modified entity-header field indicates the date and time at which
/// the sender believes the resource was last modified.
///
/// **Reference:** https://www.rfc-editor.org/rfc/rfc1945.html#section-10.10
///
#[derive(Debug, PartialEq, Eq)]
pub struct LastModified {
    name: HeaderName,
    value: HeaderValue,
}

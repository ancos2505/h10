/// ### If-Modified-Since
/// Related: Resource state
/// 304 (not modified) response will be returned without any
///          Entity-Body.
///
/// Reference: https://www.rfc-editor.org/rfc/rfc1945.html#section-10.9
///
#[derive(Debug, PartialEq, Eq)]
pub struct IfModifiedSince {
    name: String,
    value: String,
}

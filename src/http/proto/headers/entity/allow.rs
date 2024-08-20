/// ### Allow
/// Related:  It is strictly to inform the recipient of valid methods associated
///          with the resource.
///
/// Reference: https://www.rfc-editor.org/rfc/rfc1945.html#section-10.1
#[derive(Debug, PartialEq, Eq)]
pub(crate) struct Allow {
    name: String,
    value: String,
}

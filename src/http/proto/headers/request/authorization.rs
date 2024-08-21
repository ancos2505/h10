/// ### Authorization
/// Related: Authentication
///
/// Reference: https://www.rfc-editor.org/rfc/rfc1945.html#section-10.2
#[derive(Debug, PartialEq, Eq)]
pub(crate) struct Authorization {
    name: String,
    value: String,
}

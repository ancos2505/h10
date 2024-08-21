/// ### From
/// Related: Public Key Infrastructure
///
/// Reference: https://www.rfc-editor.org/rfc/rfc1945.html#section-10.8
#[derive(Debug, PartialEq, Eq)]
pub(crate) struct From {
    name: String,
    value: String,
}

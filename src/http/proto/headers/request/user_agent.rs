/// ### User-Agent
/// Related: Browser/2.14 Library/3.57
///
/// Reference: https://www.rfc-editor.org/rfc/rfc1945.html#section-10.15
#[derive(Debug, PartialEq, Eq)]
pub(crate) struct UserAgent {
    name: String,
    value: String,
}

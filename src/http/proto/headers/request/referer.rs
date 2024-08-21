/// ### Referer
/// Related:  back-links to resources for interest, logging, optimized caching,
///          etc.
///
/// Reference: https://www.rfc-editor.org/rfc/rfc1945.html#section-10.13
#[derive(Debug, PartialEq, Eq)]
pub(crate) struct Referer {
    name: String,
    value: String,
}

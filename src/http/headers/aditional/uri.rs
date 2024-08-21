use crate::http::headers::{HttpHeader, IntoHeader};

/// ### URI
/// Related: Content handling
///
///  The URI entity-header field may contain some or all of the Uniform Resource
/// Identifiers (Section 3.2) by which the Request-URI resource can be
/// identified. There is no guarantee that the resource can be accessed using
/// the URI(s) specified.
///
/// Reference: https://www.rfc-editor.org/rfc/rfc1945.html#appendix-D.2.6
///
#[derive(Debug)]
pub struct URI {
    name: String,
    value: String,
}

// TODO
// impl Default for URI {
//     fn default() -> Self {
//         Self {
//             name: "URI".into(),
//             value: "".into(),
//         }
//     }
// }

impl IntoHeader for URI {
    fn into_header(self) -> HttpHeader {
        let Self { name, value } = self;
        HttpHeader { name, value }
    }
}

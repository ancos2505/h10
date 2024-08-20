use crate::http::proto::headers::{HttpHeader, IntoHeader};

/// ### Accept-Charset
/// Related: Content handling
///
///  The Accept-Charset request-header field can be used to indicate a list of
/// preferred character sets other than the default US-ASCII and ISO-8859-1.
/// This field allows clients capable of understanding more comprehensive or
/// special-purpose character sets to signal that capability to a server which
/// is capable of representing documents in those character sets.
///
/// Reference: https://www.rfc-editor.org/rfc/rfc1945.html#appendix-D.2.2
///
#[derive(Debug, PartialEq, Eq)]
pub(crate) struct AcceptCharset {
    name: String,
    value: String,
}

// TODO
// impl Default for AcceptCharset {
//     fn default() -> Self {
//         let r#type = "*";
//         let subtype = "*";
//         Self {
//             name: "Accept-Charset".into(),
//             value: format!("UTF-8", r#type, subtype,),
//         }
//     }
// }

impl IntoHeader for AcceptCharset {
    fn into_header(self) -> HttpHeader {
        let Self { name, value } = self;
        HttpHeader { name, value }
    }
}

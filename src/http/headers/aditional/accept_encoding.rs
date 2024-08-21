use crate::http::headers::{HttpHeader, IntoHeader};

/// ### Accept-Encoding
/// Related: Content handling
///
///  The Accept-Encoding request-header field is similar to Accept, but
/// restricts the content-coding values which are acceptable in the response.
///
/// Reference: https://www.rfc-editor.org/rfc/rfc1945.html#appendix-D.2.3
///
#[derive(Debug)]
pub struct AcceptEncoding {
    name: String,
    value: String,
}

// TODO
// impl Default for AcceptEncoding {
//     fn default() -> Self {
//         let r#type = "*";
//         let subtype = "*";
//         Self {
//             name: "Accept-Encoding".into(),
//             value: format!("{}/{}", r#type, subtype,),
//         }
//     }
// }

impl IntoHeader for AcceptEncoding {
    fn into_header(self) -> HttpHeader {
        let Self { name, value } = self;
        HttpHeader { name, value }
    }
}

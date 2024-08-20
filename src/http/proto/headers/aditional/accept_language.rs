use crate::http::proto::headers::{HttpHeader, IntoHeader};

/// ### Accept-Language
/// Related: Content handling
///
///  The Accept-Language request-header field is similar to Accept, but
/// restricts the set of natural languages that are preferred as a response to
/// the request.
///
/// Reference: https://www.rfc-editor.org/rfc/rfc1945.html#appendix-D.2.4
///

pub(crate) struct AcceptLanguage {
    name: String,
    value: String,
}

// TODO
// impl Default for AcceptLanguage {
//     fn default() -> Self {
//         let r#type = "*";
//         let subtype = "*";
//         Self {
//             name: "Accept-Language".into(),
//             value: format!("{}/{}", r#type, subtype,),
//         }
//     }
// }

impl IntoHeader for AcceptLanguage {
    fn into_header(self) -> HttpHeader {
        let Self { name, value } = self;
        HttpHeader { name, value }
    }
}

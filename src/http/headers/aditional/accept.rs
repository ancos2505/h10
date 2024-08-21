use crate::http::headers::{HttpHeader, IntoHeader};

/// ### Accept
/// Related: Content handling
///
///  The Accept request-header field can be used to indicate a list of media
/// ranges which are acceptable as a response to the request. The asterisk "*"
/// character is used to group media types into ranges, with "*/*" indicating
/// all media types and "type/*" indicating all subtypes of that type. The set
/// of ranges given by the client should represent what types are acceptable
/// given the context of the request.
///
/// Reference: https://www.rfc-editor.org/rfc/rfc1945.html#appendix-D.2.1
///
#[derive(Debug, PartialEq, Eq)]
pub struct Accept {
    name: String,
    value: String,
}

// TODO
// impl Default for Accept {
//     fn default() -> Self {
//         let r#type = "*";
//         let subtype = "*";
//         Self {
//             name: "Accept".into(),
//             value: format!("{}/{}", r#type, subtype,),
//         }
//     }
// }

impl IntoHeader for Accept {
    fn into_header(self) -> HttpHeader {
        let Self { name, value } = self;
        HttpHeader { name, value }
    }
}

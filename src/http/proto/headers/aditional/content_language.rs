use crate::http::proto::headers::{HttpHeader, IntoHeader};

/// ### Content-Language
/// Related: Content handling
///
///  The Content-Language entity-header field describes the natural language(s)
/// of the intended audience for the enclosed entity. Note that this may not be
/// equivalent to all the languages used within the entity.
///
/// Reference: https://www.rfc-editor.org/rfc/rfc1945.html#appendix-D.2.5
///
pub(crate) struct ContentLanguage {
    name: String,
    value: String,
}

// TODO
// impl Default for ContentLanguage {
//     fn default() -> Self {
//         let r#type = "*";
//         let subtype = "*";
//         Self {
//             name: "Content-Language".into(),
//             value: format!("{}/{}", r#type, subtype,),
//         }
//     }
// }

impl IntoHeader for ContentLanguage {
    fn into_header(self) -> HttpHeader {
        let Self { name, value } = self;
        HttpHeader { name, value }
    }
}

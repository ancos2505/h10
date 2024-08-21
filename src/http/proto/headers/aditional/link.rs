use crate::http::proto::headers::{HttpHeader, IntoHeader};

/// ### Link
/// Related: Content handling
///
///  The Link entity-header field provides a means for describing a relationship
/// between the entity and some other resource. An entity may include multiple
/// Link values. Links at the metainformation level typically indicate
/// relationships like hierarchical structure and navigation paths.
///
/// Reference: https://www.rfc-editor.org/rfc/rfc1945.html#appendix-D.2.6
///

pub(crate) struct Link {
    name: String,
    value: String,
}

// TODO
// impl Default for Link {
//     fn default() -> Self {
//         let r#type = "*";
//         let subtype = "*";
//         Self {
//             name: "Link".into(),
//             value: format!("{}/{}", r#type, subtype,),
//         }
//     }
// }

impl IntoHeader for Link {
    fn into_header(self) -> HttpHeader {
        let Self { name, value } = self;
        HttpHeader { name, value }
    }
}

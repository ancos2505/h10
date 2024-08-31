use crate::http::headers::{HeaderEntry, HeaderName, HeaderValue, IntoHeader};

/// ### Title
/// Related: Content handling
///
///  The Title entity-header field indicates the title of the entity.
///
/// Reference: https://www.rfc-editor.org/rfc/rfc1945.html#appendix-D.2.6
///
#[derive(Debug)]
pub struct Title {
    name: HeaderName,
    value: HeaderValue,
}

// TODO
// impl Default for Title {
//     fn default() -> Self {
//         let r#type = "*";
//         let subtype = "*";
//         Self {
//             name: "Title".into(),
//             value: "Example Title".into(),
//         }
//     }
// }

impl IntoHeader for Title {
    fn into_header(self) -> HeaderEntry {
        let Self { name, value } = self;
        HeaderEntry { name, value }
    }
}

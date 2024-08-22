use crate::http::{
    headers::{HttpHeader, IntoHeader},
    result::H10LibResult,
    url::UrlParts,
};

/// ### Location header
/// Related: Indicate a redirection
///
/// Reference: https://www.rfc-editor.org/rfc/rfc1945.html#section-10.11
#[derive(Debug, PartialEq, Eq)]
pub struct Location {
    name: String,
    value: String,
}
impl Location {
    pub fn from_str<S: AsRef<str>>(url: S) -> H10LibResult<Self> {
        Ok(Self {
            name: "Location".into(),
            value: UrlParts::parse(url.as_ref())?.to_string(),
        })
    }
}
// impl Default for Location {
//     fn default() -> Self {
//         Self {
//             name: "Location".into(),
//             value: "#".into(),
//             ),
//         }
//     }
// }

impl IntoHeader for Location {
    fn into_header(self) -> HttpHeader {
        let Self { name, value } = self;
        HttpHeader { name, value }
    }
}

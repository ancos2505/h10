use std::str::FromStr;

use crate::http::{
    headers::{HeaderEntry, HeaderName, HeaderValue, IntoHeader},
    result::H10LibError,
};

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
    name: HeaderName,
    value: HeaderValue,
}

impl Default for Accept {
    fn default() -> Self {
        Self {
            name: HeaderName::new_unchecked("Accept"),
            value: HeaderValue::new_unchecked("Not_Defined"),
        }
    }
}

impl FromStr for Accept {
    type Err = H10LibError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let entry: HeaderEntry = s.parse()?;
        Ok(entry.into())
    }
}

impl From<HeaderEntry> for Accept {
    fn from(value: HeaderEntry) -> Self {
        let HeaderEntry { name, value } = value;
        Self { name, value }
    }
}

impl IntoHeader for Accept {
    fn into_header(self) -> HeaderEntry {
        let Self { name, value } = self;
        HeaderEntry { name, value }
    }
}

use std::str::FromStr;

use crate::http::result::H10LibError;

/// ### Request Body
/// Should compilant with RFC 1867 - Form-based File Upload in HTML
///
/// Reference: https://www.rfc-editor.org/rfc/rfc1867
///
#[derive(Debug, PartialEq, Eq)]
pub struct Body(String);

impl FromStr for Body {
    type Err = H10LibError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let collected: String = s.chars().filter(|c| *c as u8 != 0).collect();
        Ok(Self(collected.to_owned()))
    }
}

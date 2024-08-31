use std::{fmt::Display, rc::Rc, str::FromStr};

use crate::http::result::H10LibError;

/// ### Request Body
/// Should compilant with RFC 1867 - Form-based File Upload in HTML
///
/// Reference: https://www.rfc-editor.org/rfc/rfc1867
///
#[derive(Debug, PartialEq, Eq)]
pub struct Body(Rc<str>);

impl Body {
    pub fn new_unchecked(s: &str) -> Self {
        Self(s.into())
    }

    pub fn len(&self) -> usize {
        self.0.len()
    }
}

impl FromStr for Body {
    type Err = H10LibError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(Self(s.into()))
    }
}

impl Display for Body {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

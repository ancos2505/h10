use std::{ops::Deref, rc::Rc};

use crate::http::result::H10LibResult;

#[derive(Debug, PartialEq, Eq)]
pub struct UrlPath(Rc<str>);

impl Default for UrlPath {
    fn default() -> Self {
        Self("/".into())
    }
}
impl Deref for UrlPath {
    type Target = str;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl UrlPath {
    pub fn parse<S: AsRef<str>>(s: S) -> H10LibResult<Self> {
        let input = s.as_ref();
        Ok(Self(input.into()))
    }
}

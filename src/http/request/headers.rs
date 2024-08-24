use std::{ops::Deref, rc::Rc, str::FromStr};

use crate::http::result::{H10LibError, H10LibResult};

#[derive(Debug, Default, PartialEq, Eq)]
pub struct Headers(Vec<HeaderEntry>);
impl Headers {
    pub fn parse(s: Option<&str>) -> H10LibResult<Self> {
        let headers_str = match s {
            Some(s) => {
                if s.len() > 0 {
                    s
                } else {
                    return Ok(Headers::default());
                }
            }
            None => return Ok(Headers::default()),
        };

        let mut headers = vec![];

        let mut iter = headers_str.split("\r\n");
        while let Some(entry) = iter.next() {
            headers.push(entry.parse()?);
        }
        Ok(Self(headers))
    }
}

#[derive(Debug, PartialEq, Eq)]
pub struct HeaderEntry {
    name: HeaderName,
    value: HeaderValue,
}
impl FromStr for HeaderEntry {
    type Err = H10LibError;

    fn from_str(input: &str) -> Result<Self, Self::Err> {
        if let Some((key, value)) = input.split_once(":") {
            Ok(Self {
                name: key.trim().parse()?,
                value: value.trim().parse()?,
            })
        } else {
            Err(H10LibError::RequestParser(
                "Malformed HTTP Header entry".into(),
            ))
        }
    }
}

#[derive(Debug, PartialEq, Eq)]
pub struct HeaderName(Rc<str>);

impl Deref for HeaderName {
    type Target = str;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl FromStr for HeaderName {
    type Err = H10LibError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(Self(s.into()))
    }
}

#[derive(Debug, PartialEq, Eq)]
pub struct HeaderValue(Rc<str>);

impl Deref for HeaderValue {
    type Target = str;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl FromStr for HeaderValue {
    type Err = H10LibError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(Self(s.into()))
    }
}

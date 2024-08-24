use std::{ops::Deref, rc::Rc, str::FromStr};

use crate::http::result::{H10LibError, H10LibResult};

#[derive(Debug, Default, PartialEq, Eq)]
pub struct QueryString(Vec<QsEntry>);
impl QueryString {
    pub fn parse(s: Option<&str>) -> H10LibResult<Self> {
        let input = match s {
            Some(s) => s,
            None => return Ok(QueryString::default()),
        };

        let mut headers = vec![];

        let mut iter = input.split("\r\n");
        while let Some(entry) = iter.next() {
            headers.push(entry.parse()?);
        }
        Ok(Self(headers))
    }
    pub fn get(&self, name: &str) -> Option<&QsEntry> {
        for entry in &self.0 {
            if &*entry.name == name {
                return Some(entry);
            }
        }
        None
    }
}
impl FromStr for QueryString {
    type Err = H10LibError;

    fn from_str(input: &str) -> Result<Self, Self::Err> {
        let mut headers = vec![];

        let mut iter = input.split("\r\n");
        while let Some(entry) = iter.next() {
            headers.push(entry.parse()?);
        }
        Ok(Self(headers))
    }
}

#[derive(Debug, PartialEq, Eq)]
pub struct QsEntry {
    name: QsName,
    value: QsValue,
}

impl QsEntry {
    pub fn name(&self) -> &QsName {
        &self.name
    }

    pub fn value(&self) -> &QsValue {
        &self.value
    }
}

impl FromStr for QsEntry {
    type Err = H10LibError;

    fn from_str(input: &str) -> Result<Self, Self::Err> {
        if let Some((key, value)) = input.split_once(":") {
            Ok(Self {
                name: key.parse()?,
                value: value.parse()?,
            })
        } else {
            Err(H10LibError::RequestParser(
                "Malformed Query String entry".into(),
            ))
        }
    }
}

#[derive(Debug, PartialEq, Eq)]
pub struct QsName(Rc<str>);

impl Deref for QsName {
    type Target = str;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl FromStr for QsName {
    type Err = H10LibError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(Self(s.into()))
    }
}

#[derive(Debug, PartialEq, Eq)]
pub struct QsValue(Rc<str>);

impl Deref for QsValue {
    type Target = str;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl FromStr for QsValue {
    type Err = H10LibError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(Self(s.into()))
    }
}

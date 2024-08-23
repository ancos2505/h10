use std::{borrow::Cow, cell::RefCell, marker::PhantomData, ops::Deref, rc::Rc};

use crate::{
    constants::AsciiWhiteSpace,
    http::{
        method::Method,
        result::{H10LibError, H10LibResult},
        url_parts::UrlParts2,
        version::Version,
    },
};

#[derive(Debug, Default)]
pub struct RequestParsed<'a> {
    raw_request: Cow<'a, str>,
    pub http_version: Option<Version>,
    method: Option<Method>,
    // pub url: Option<UrlPartsParsed<'a>>,
    pub url: Option<UrlParts2>,
    headers: Option<Headers<'a>>,
    body: Option<BodyParsed<'a>>,
}
#[derive(Debug, Default)]
pub struct Headers<'a>(Vec<(HeaderName<'a>, HeaderValue<'a>)>);

#[derive(Debug)]
pub struct HeaderName<'a>(&'a str);

#[derive(Debug)]
pub struct HeaderValue<'a>(&'a str);

#[derive(Debug)]
pub struct BodyParsed<'a>(&'a str);

#[derive(Debug)]
pub struct UrlPartsParsed<'a> {
    raw_url: &'a str,
    pub path: Option<PathParsed<'a>>,
    pub query: QueryString<'a>,
    pub fragment: Option<FragmentParsed<'a>>,
}

#[derive(Debug)]
pub struct PathParsed<'a>(&'a str);

impl Deref for PathParsed<'_> {
    type Target = str;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[derive(Debug)]
pub struct FragmentParsed<'a>(&'a str);

#[derive(Debug, Default)]
pub struct QueryString<'a>(Vec<QsEntry<'a>>);
impl QueryString<'_> {
    // pub fn remove_entry(&mut self, key: &str) -> Option<QsEntry<'_>> {
    //     let idx = self.find(key)?;
    //     safe_swap_remove(&mut self.0, idx)
    // }
    pub fn get(&self, key: &str) -> Option<&QsEntry<'_>> {
        let idx = self.find(key)?;
        self.0.get(idx)
    }
    pub fn find(&self, key: &str) -> Option<usize> {
        for (idx, entry) in self.0.iter().enumerate() {
            if **&entry.name == key {
                return Some(idx);
            } else {
                continue;
            }
        }
        None
    }
    pub fn contains_key(&self, key: &str) -> bool {
        self.0
            .iter()
            .filter(|entry| &**entry.name == key)
            .next()
            .is_some()
    }
    // pub fn insert(&mut self, key: K, value: V) -> Option<V> {
    //     None
    // }
}

#[derive(Debug)]
pub struct QsEntry<'a> {
    pub name: QsName<'a>,
    pub value: QsValue<'a>,
}

#[derive(Debug)]
pub struct QsName<'a>(&'a str);

impl<'a> Deref for QsName<'a> {
    type Target = &'a str;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

#[derive(Debug)]
pub struct QsValue<'a>(&'a str);
impl<'a> Deref for QsValue<'a> {
    type Target = &'a str;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

pub trait RequestParserState {}

impl RequestParserState for Initial {}
impl RequestParserState for BeforeParse {}
impl RequestParserState for AfterMethod {}
impl RequestParserState for AfterUrl {}
impl RequestParserState for AfterHttpVersion {}
impl RequestParserState for AfterHeaders {}
impl RequestParserState for Complete {}

// States
#[derive(Debug)]
pub struct Initial;

#[derive(Debug)]
pub struct BeforeParse;

#[derive(Debug)]
pub struct AfterMethod;

#[derive(Debug)]
pub struct AfterUrl;

#[derive(Debug)]
pub struct AfterHttpVersion;

#[derive(Debug)]
pub struct AfterHeaders;

#[derive(Debug)]
pub struct Complete;

// Main parser struct
#[derive(Debug)]
pub struct RequestParser<'a, State = Initial>
where
    State: RequestParserState,
{
    position: usize,
    request: Rc<RefCell<RequestParsed<'a>>>,
    _state: PhantomData<State>,
}

impl<S: RequestParserState> RequestParser<'_, S> {
    fn advance(&mut self, n: usize) {
        self.position += n;
    }

    fn consume_whitespace(&mut self) -> H10LibResult<()> {
        if Self::get_remaining(&self.request.borrow().raw_request, self.position)?
            .starts_with(AsciiWhiteSpace::as_str())
        {
            self.advance(AsciiWhiteSpace::len());
            Ok(())
        } else {
            return Err(H10LibError::RequestParser(format!(
                "Expected `{}`",
                AsciiWhiteSpace
            )));
        }
    }

    fn get_remaining(input: &str, pos: usize) -> H10LibResult<&str> {
        input
            .split_at_checked(pos)
            .map(|(_, remaining)| remaining)
            .ok_or(H10LibError::RequestParser("Unexpected empty".to_string()))
    }
}

impl<'a> RequestParser<'a, Initial> {
    pub fn new(raw_request: Cow<'a, str>) -> H10LibResult<RequestParser<'a, BeforeParse>> {
        Ok(RequestParser {
            position: 0,
            _state: PhantomData,
            request: Rc::new(RefCell::new(RequestParsed {
                raw_request,
                ..Default::default()
            })),
        })
    }
}

impl<'a> RequestParser<'a, BeforeParse> {
    pub fn method(mut self) -> H10LibResult<RequestParser<'a, AfterMethod>> {
        let method = {
            let request = self.request.borrow();
            let remaining = Self::get_remaining(&request.raw_request, self.position)?;

            if let Some(pos_end) = remaining.find("HTTP") {
                let maybe_method_str = remaining.split_at_checked(pos_end).and_then(|(s, _)| {
                    s.split_once(AsciiWhiteSpace::as_str())
                        .map(|(method, _)| method)
                });

                maybe_method_str
                    .ok_or_else(|| H10LibError::RequestParser("Method not found".to_string()))?
                    .parse::<Method>()?
            } else {
                return Err(H10LibError::RequestParser(format!(
                    "Expected `{}`",
                    stringify!(Method)
                )));
            }
        };

        let method_str_len = method.as_str().len();

        {
            let mut request = self.request.borrow_mut();

            request.method = Some(method);
        }

        Ok(RequestParser {
            position: self.position + method_str_len,
            _state: PhantomData,
            request: self.request,
        })
    }
}

impl<'a> RequestParser<'a, AfterMethod> {
    pub fn url(mut self) -> H10LibResult<RequestParser<'a, AfterUrl>> {
        let mut maybe_url_len = 0;
        let url_parts = {
            let request = self.request.borrow();
            let remaining = Self::get_remaining(&request.raw_request, self.position)?;

            if let Some(pos_end) = remaining.find("HTTP") {
                let maybe_url_str = remaining.split_at_checked(pos_end).and_then(|(s, _)| {
                    s.split_once(AsciiWhiteSpace::as_str())
                        .map(|(_, url_parts)| url_parts)
                });
                if let Some(url) = &maybe_url_str {
                    maybe_url_len = url.len();
                }
                let s = maybe_url_str
                    .ok_or_else(|| H10LibError::RequestParser("Url not found".to_string()))?;
                // TODO
                UrlParts2::parse(s)?
            } else {
                return Err(H10LibError::RequestParser(format!(
                    "Expected `{}`",
                    stringify!(UrlParts2)
                )));
            }
        };

        {
            let mut request = self.request.borrow_mut();

            request.url = Some(url_parts);
        }

        Ok(RequestParser {
            position: self.position + maybe_url_len,
            _state: PhantomData,
            request: self.request,
        })
    }
}

impl<'a> RequestParser<'a, AfterUrl> {
    pub fn version(mut self) -> H10LibResult<RequestParser<'a, Complete>> {
        let version = {
            let request = self.request.borrow();
            let remaining = Self::get_remaining(&request.raw_request, self.position)?;

            if let Some(pos_end) = remaining.find("HTTP") {
                let maybe_version_str = remaining.split_at_checked(pos_end).and_then(|(_, s)| {
                    s.split_once("\r\n")
                        .map(|(http_version_str, _)| http_version_str)
                });

                maybe_version_str
                    .ok_or_else(|| H10LibError::RequestParser("Version not found".to_string()))?
                    .parse::<Version>()?
            } else {
                return Err(H10LibError::RequestParser(format!(
                    "Expected `{}`",
                    stringify!(Version)
                )));
            }
        };

        let method_str_len = version.as_str().len();

        {
            let mut request = self.request.borrow_mut();

            request.http_version = Some(version);
        }

        Ok(RequestParser {
            position: self.position + method_str_len,
            _state: PhantomData,
            request: self.request,
        })
    }
}

impl<'a> RequestParser<'a, Complete> {
    pub fn finish(&self) -> H10LibResult<Rc<RefCell<RequestParsed<'a>>>> {
        Ok(Rc::clone(&self.request))
    }
}

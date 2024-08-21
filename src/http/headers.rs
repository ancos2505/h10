mod aditional;
mod common;
mod compat;
mod entity;
mod request;
mod response;

use std::fmt::{Debug, Display};

pub use self::{
    aditional::{
        Accept, AcceptCharset, AcceptEncoding, AcceptLanguage, ContentLanguage, Link, MIMEVersion,
        RetryAfter, Title, URI,
    },
    common::{Date, Pragma},
    compat::Connection,
    entity::{Allow, ContentEncoding, ContentLength, ContentType, Expires, LastModified},
    request::{Authorization, From, IfModifiedSince, Referer, UserAgent},
    response::{Location, Server, WWWAuthenticate},
};

pub trait IntoHeader {
    fn into_header(self) -> HttpHeader;
}

#[derive(Debug, PartialEq, Eq, Hash)]
pub struct HttpHeader {
    pub name: String,
    pub value: String,
}
impl Display for HttpHeader {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}: {}", self.name, self.value)
    }
}

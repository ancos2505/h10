#[derive(Debug, PartialEq, Eq)]
pub(crate) struct Host(String);

impl From<Host> for (String, String) {
    fn from(value: Host) -> Self {
        ("Host".into(), value.0)
    }
}

#[derive(Debug, PartialEq, Eq)]
pub(crate) struct UserAgent(String);

impl From<UserAgent> for (String, String) {
    fn from(value: UserAgent) -> Self {
        ("User-Agent".into(), value.0)
    }
}

#[derive(Debug, PartialEq, Eq)]
pub(crate) struct Accept(String);

impl From<Accept> for (String, String) {
    fn from(value: Accept) -> Self {
        ("Accept".into(), value.0)
    }
}

#[derive(Debug, PartialEq, Eq)]
pub(crate) struct Server(String);

#[derive(Debug, PartialEq, Eq)]
pub(crate) struct Date(String);

#[derive(Debug, PartialEq, Eq)]
pub(crate) struct Connection(String);

#[derive(Debug, PartialEq, Eq)]
pub(crate) struct ContentType(String);
impl ContentType {
    pub fn html() -> Self {
        Self("Content-Type: text/html; charset=UTF-8".into())
    }
    pub fn javascript() -> Self {
        Self("Content-Type: application/javascript; charset=UTF-8".into())
    }
    pub fn json() -> Self {
        Self("Content-Type: application/json; charset=UTF-8".into())
    }
}
#[derive(Debug, PartialEq, Eq)]
pub(crate) struct ContentLength(usize);

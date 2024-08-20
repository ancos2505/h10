use std::{fmt::Display, str::FromStr};

use crate::result::H10ServerError;

#[derive(Debug, Default, PartialEq, Eq)]
pub enum Version {
    #[default]
    Http1_0,
    // Http1_1,
}
impl Display for Version {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut output = "".to_owned();
        output.push_str("HTTP/");
        match self {
            Version::Http1_0 => output.push_str("1.0"),
            // Version::Http1_1 => output.push_str("1.1"),
        };
        write!(f, "{}", output)
    }
}

impl FromStr for Version {
    type Err = H10ServerError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        dbg!(s);
        match s {
            "HTTP/1.0" => Ok(Self::Http1_0),
            _ => Err(H10ServerError("Version not supported".into())),
        }
    }
}

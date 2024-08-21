use std::fmt::{Debug, Display};

#[derive(Debug, PartialEq, Eq)]
pub enum StatusCode {
    OK,
    Created,
    Accepted,
    NoContent,
    MovedPermanently,
    MovedTemporarily,
    NotModified,
    BadRequest,
    Unauthorized,
    Forbidden,
    NotFound,
    InternalServerError,
    NotImplemented,
    BadGateway,
    ServiceUnavailable,
}

impl Display for StatusCode {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let output = match self {
            StatusCode::OK => "200 OK",
            StatusCode::Created => "201 Created",
            StatusCode::Accepted => "202 Accepted",
            StatusCode::NoContent => "204 No Content",
            StatusCode::MovedPermanently => "301 Moved Permanently",
            StatusCode::MovedTemporarily => "302 Moved Temporarily",
            StatusCode::NotModified => "304 Not Modified",
            StatusCode::BadRequest => "400 Bad Request",
            StatusCode::Unauthorized => "401 Unauthorized",
            StatusCode::Forbidden => "403 Forbidden",
            StatusCode::NotFound => "404 Not Found",
            StatusCode::InternalServerError => "500 Internal Server Error",
            StatusCode::NotImplemented => "501 Not Implemented",
            StatusCode::BadGateway => "502 Bad Gateway",
            StatusCode::ServiceUnavailable => "503 Service Unavailable",
        };

        write!(f, "{}", output)
    }
}

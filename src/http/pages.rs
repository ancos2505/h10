use std::borrow::Cow;

use crate::http::proto::{response::Response, status_code::StatusCode};

use super::proto::request::Request;

mod error_404;
mod root;

pub struct Endpoint;

impl<'a> Endpoint {
    pub fn dispatcher(request: Cow<'a, str>) -> String {
        use super::pages::{error_404::error_404, root::root};
        use crate::result::H10ServerError;
        let request = match Request::parse(request) {
            Ok(req) => req,
            Err(err) => {
                let status = match err {
                    H10ServerError::MethodNotSupported
                    | H10ServerError::InvalidInputData(_)
                    | H10ServerError::VersionNotSupported => StatusCode::BadRequest,
                    H10ServerError::ParseFloatError(_) => StatusCode::InternalServerError,
                    H10ServerError::SystemTimeError(_) => StatusCode::InternalServerError,
                    H10ServerError::ParseIntError(_) => StatusCode::InternalServerError,
                    H10ServerError::IoError(_) => StatusCode::InternalServerError,
                    H10ServerError::Custom(_) => StatusCode::InternalServerError,
                };
                eprintln!("{err}");
                return Response::new(status).to_string();
            }
        };

        let maybe_response = request
            .path
            .and_then(|url| url.path)
            .and_then(|path| match &*path {
                "" => root().ok(),
                "/" => root().ok(),
                _ => Some(error_404()),
            })
            .or_else(|| root().ok())
            .map(|res| res.to_string());
        if let Some(response) = maybe_response {
            return response;
        } else {
            error_404().to_string()
        }
    }
}

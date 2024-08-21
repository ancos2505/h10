use std::borrow::Cow;

use h10::http::{
    request::Request, response::Response, result::H10LibError, status_code::StatusCode,
};

mod error_404;
mod root;

pub struct Endpoint;

impl<'a> Endpoint {
    pub fn dispatcher(request: Cow<'a, str>) -> String {
        use super::pages::{error_404::error_404, root::root};
        let request = match Request::parse(request) {
            Ok(req) => req,
            Err(err) => {
                let status = match err {
                    H10LibError::MethodNotSupported
                    | H10LibError::InvalidInputData(_)
                    | H10LibError::VersionNotSupported => StatusCode::BadRequest,
                    H10LibError::ParseFloatError(_) => StatusCode::InternalServerError,
                    H10LibError::SystemTimeError(_) => StatusCode::InternalServerError,
                    H10LibError::ParseIntError(_) => StatusCode::InternalServerError,
                    H10LibError::IoError(_) => StatusCode::InternalServerError,
                    H10LibError::Custom(_) => StatusCode::InternalServerError,
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

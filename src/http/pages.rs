use std::borrow::Cow;

use super::proto::request::Request;

mod error_404;
mod root;

pub struct Endpoint;

impl<'a> Endpoint {
    pub fn dispatcher(request: Cow<'a, str>) -> String {
        use super::pages::{error_404::error_404, root::root};
        let request = match Request::parse(request) {
            Ok(req) => req,
            Err(err) => {
                eprintln!("{err}");
                return error_404().to_string();
            }
        };

        dbg!(&request.path);
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

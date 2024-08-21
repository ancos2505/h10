mod error_404;
mod root;

use std::borrow::Cow;

use h10::http::{request::Request, result::H10LibError, status_code::StatusCode, version::Version};

use crate::{
    server::{CliHttp10StrictMode, ServerResponse},
    CLI_ARGS,
};

pub struct Endpoint;

impl<'a> Endpoint {
    pub fn dispatcher(request_str: Cow<'a, str>) -> ServerResponse {
        use super::pages::{error_404::error_404, root::root};
        let request = match Request::parse(&request_str) {
            Ok(req) => req,
            Err(err) => {
                eprintln!("{err}");
                return ServerResponse::new(err.into());
            }
        };

        if let Some(cli_data) = CLI_ARGS.get() {
            if cli_data.h10_strict_mode == CliHttp10StrictMode::Enabled
                && request.http_version != Version::Http1_0
            {
                let err = H10LibError::VersionNotSupported;
                return ServerResponse::new(err.into());
            }
        }

        let maybe_response = request
            .url_parts
            .path
            .and_then(|path| match &*path {
                "" => root().ok(),
                "/" => root().ok(),
                _ => Some(error_404()),
            })
            .or_else(|| root().ok())
            .map(|res| res);
        if let Some(response) = maybe_response {
            return response;
        } else {
            error_404()
        }
    }
}

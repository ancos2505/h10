mod error_404;
mod root;
mod styles_css;

use std::borrow::Cow;

use h10::http::{request::Request, result::H10LibError, version::Version};

use crate::{
    server::{CliHttp10StrictMode, ServerResponse},
    CLI_ARGS,
};

use self::styles_css::styles_css;

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

        let maybe_response = match &request.url_parts.path {
            Some(path) => match &**path {
                "" => root(request).ok(),
                "/" => root(request).ok(),
                "/assets/styles.css" => styles_css().ok(),
                _ => Some(error_404()),
            },
            None => Some(error_404()),
        };

        if let Some(response) = maybe_response {
            return response;
        } else {
            error_404()
        }
    }
}

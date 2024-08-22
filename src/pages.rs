mod error_404;
mod root;
mod styles_css;

use std::borrow::Cow;

use h10::http::{
    request::{parser::RequestParser, Request},
    result::H10LibError,
    version::Version,
};

use crate::{
    server::{CliHttp10StrictMode, ServerResponse},
    CLI_ARGS,
};

use self::styles_css::styles_css;

pub struct Endpoint;

impl<'a> Endpoint {
    pub fn dispatcher(request_str: Cow<'a, str>) -> ServerResponse {
        use super::pages::{error_404::error_404, root::root};

        let mut parser = RequestParser::new(request_str)
            .and_then(|p| p.method())
            .and_then(|p| p.url())
            .and_then(|p| p.version());
        let res_request = match parser {
            Ok(req) => req.finish(),
            Err(err) => {
                eprintln!("{err}");
                return ServerResponse::new(err.into());
            }
        };

        if let Some(cli_data) = CLI_ARGS.get() {
            if let Ok(ref request_parsed) = res_request {
                let req = request_parsed.borrow();
                if let Some(http_version) = req.http_version.as_ref() {
                    if cli_data.h10_strict_mode == CliHttp10StrictMode::Enabled
                        && *http_version != Version::Http1_0
                    {
                        let err = H10LibError::VersionNotSupported;
                        return ServerResponse::new(err.into());
                    }
                }
            }
        }

        let maybe_response = if let Ok(ref request_parsed) = res_request {
            let req = request_parsed.borrow();
            // dbg!(&req);
            if let Some(url) = req.url.as_ref() {
                if let Some(path) = &url.path {
                    match path.trim() {
                        "" => root(req).ok(),
                        "/" => root(req).ok(),
                        "/assets/styles.css" => styles_css().ok(),
                        _ => Some(error_404()),
                    }
                } else {
                    Some(error_404())
                }
            } else {
                todo!();
                Some(error_404())
            }
        } else {
            Some(error_404())
        };

        if let Some(response) = maybe_response {
            return response;
        } else {
            error_404()
        }
    }
}

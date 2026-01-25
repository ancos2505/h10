mod error_404;
mod pico_min_css;
mod root;
mod styles_css;

use h10::{
    error,
    http::{request::Request, result::H10LibError, version::Version},
};
use pico_min_css::pico_min_css;

use crate::{
    CLI_ARGS,
    server::{CliHttp10StrictMode, CliVerboseMode, ServerResponse},
};

use self::styles_css::styles_css;

pub struct Endpoint;

impl Endpoint {
    pub fn dispatcher(raw_request: &[u8]) -> ServerResponse {
        use super::pages::{error_404::error_404, root::root};

        let request = match Request::parse(raw_request) {
            Ok(req) => req,
            Err(err) => {
                // TODO: Implement log log level inside message
                error!(format!("Error: {err}"));
                return ServerResponse::new(err.into());
            }
        };

        if let Some(cli_data) = CLI_ARGS.get() {
            if cli_data.h10_strict_mode == CliHttp10StrictMode::Enabled
                && *request.http_version() != Version::Http1_0
            {
                let err = H10LibError::VersionNotSupported;
                return ServerResponse::new(err.into());
            }
        }

        let res = match &**request.path() {
            "/" => root(request),
            "/assets/styles.css" => styles_css(),
            "/assets/pico.min.css" => pico_min_css(),
            _ => error_404(),
        };

        match res {
            Ok(response) => return response,
            Err(err) => ServerResponse::new(err.into()),
        }
    }
}

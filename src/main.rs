mod http;
mod result;

use crate::{http::HttpServer, result::ServerResult};

fn main() -> ServerResult<()> {
    HttpServer::run()
}

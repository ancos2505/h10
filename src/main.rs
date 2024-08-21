mod http;
mod result;

use crate::{http::HttpServer, result::ServerResult};

const MAX_ACTIVE_SESSIONS: usize = 5_000;

fn main() -> ServerResult<()> {
    HttpServer::run()
}

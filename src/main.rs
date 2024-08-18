mod http;
mod result;

use crate::{http::HttpServer, result::AppResult};

fn main() -> AppResult<()> {
    HttpServer::run()
}

mod pages;
mod server;

use std::sync::OnceLock;

use crate::server::{Cli, HttpServer, ServerResult};

static HTTP10_STRICT_MODE: OnceLock<bool> = OnceLock::new();

static CLI_ARGS: OnceLock<Cli> = OnceLock::new();

fn main() -> ServerResult<()> {
    let cli = Cli::parse()?;

    CLI_ARGS.get_or_init(|| cli);

    HttpServer::run()
}

mod pages;
mod server;

use std::{
    process::ExitCode,
    sync::{atomic::AtomicUsize, OnceLock},
};

use crate::server::{Cli, HttpServer, ServerResult};

// Unsafe
static ROOT_PAGER_COUNTER: AtomicUsize = AtomicUsize::new(0);

static HTTP10_STRICT_MODE: OnceLock<bool> = OnceLock::new();

static CLI_ARGS: OnceLock<Cli> = OnceLock::new();

pub(crate) const FOUR_KBYTES: usize = 1024 * 4;

pub(crate) const MAX_ACTIVE_SESSIONS: usize = 5_000;

fn main() -> ExitCode {
    match smain() {
        Ok(_) => ExitCode::SUCCESS,
        Err(err) => match err {
            server::ServerError::InvalidCLiArgs(arg) => {
                eprintln!("Error: unexpected argument '{arg}'\n");
                Cli::usage();
                ExitCode::FAILURE
            }
            server::ServerError::H10LibError(_)
            | server::ServerError::StdIoError(_)
            | server::ServerError::AddrParseError(_)
            | server::ServerError::PoisonErrorRwLockReadGuard
            | server::ServerError::PortParseError
            | server::ServerError::InvalidLogLevel
            | server::ServerError::Custom(_) => ExitCode::from(2),
        },
    }
}

fn smain() -> ServerResult<()> {
    let cli = Cli::parse()?;
    CLI_ARGS.get_or_init(|| cli);

    HttpServer::run()?;
    Ok(())
}

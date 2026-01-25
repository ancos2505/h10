use h10::log::LogLevel;

use super::traits::ArgName;

pub(crate) type CliLogLevel = LogLevel;

impl ArgName for CliLogLevel {
    fn arg_name() -> String {
        "--log-level".into()
    }
}

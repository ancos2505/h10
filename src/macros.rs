#[macro_export]
macro_rules! error {
    ($message:expr) => {
        if let Some(cli_data) = CLI_ARGS.get() {
            let level = $crate::log::LogLevel::Error;
            $crate::log!(level, $message);
        }
    };
}

#[macro_export]
macro_rules! warn {
    ($message:expr) => {
        if let Some(cli_data) = CLI_ARGS.get() {
            let level = $crate::log::LogLevel::Warn;
            $crate::log!(level, $message);
        }
    };
}

#[macro_export]
macro_rules! info {
    ($message:expr) => {
        if let Some(cli_data) = CLI_ARGS.get() {
            let level = $crate::log::LogLevel::Info;
            $crate::log!(level, $message);
        }
    };
}

#[macro_export]
macro_rules! debug {
    ($message:expr) => {
        if let Some(cli_data) = CLI_ARGS.get() {
            let level = $crate::log::LogLevel::Debug;
            $crate::log!(level, $message);
        }
    };
}

#[macro_export]
macro_rules! trace {
    ($message:expr) => {
        if let Some(cli_data) = CLI_ARGS.get() {
            let level = $crate::log::LogLevel::Trace;
            $crate::log!(level, $message);
        }
    };
}

#[macro_export]
macro_rules! log {
    ($level:expr,$message:expr) => {
        if let Some(cli_data) = CLI_ARGS.get() {
            let level = $level;
            if cli_data.log_level == level {
                println!("{}: {}", level, $message);
            }
        }
    };
}

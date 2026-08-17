use std::str::FromStr;

use log::{Level, Metadata, Record};
use once_cell::sync::Lazy;

use crate::ecosystem::environment_variables::cdrm_env_variable::CdrmEnvVariable;

/// ## Conundrum Logging & Error Handling
/// Deprecated now that the Conundrum server and more persistant storage is
/// becoming available.
pub struct Logger {
    level: Level,
}

impl Default for Logger {
    fn default() -> Self {
        let level = CdrmEnvVariable::LogLevel.read()
                                             .map(|res| Level::from_str(res.as_str()).unwrap_or_else(|_| Level::Info))
                                             .unwrap_or_else(|_| Level::Info);
        Logger { level }
    }
}

impl log::Log for Logger {
    fn enabled(&self, metadata: &Metadata) -> bool {
        metadata.level() <= self.level
    }

    fn log(&self, record: &Record) {
        if self.enabled(record.metadata()) {
            println!("{} - {}", record.level(), record.args());
        }
    }

    fn flush(&self) {}
}

pub static LOGGER: Lazy<Logger> = Lazy::new(Logger::default);

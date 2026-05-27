use log::Level;
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
use strum::IntoEnumIterator;

#[derive(Serialize, Deserialize, strum_macros::EnumIter, strum_macros::Display, Clone, JsonSchema)]
pub enum LogLevel {
    #[serde(rename = "ERROR")]
    #[strum(to_string = "ERROR")]
    Error,
    #[serde(rename = "WARN")]
    #[strum(to_string = "WARN")]
    Warn,
    #[serde(rename = "INFO")]
    #[strum(to_string = "INFO")]
    Info,
    #[serde(rename = "DEBUG")]
    #[strum(to_string = "DEBUG")]
    Debug,
    #[serde(rename = "TRACE")]
    #[strum(to_string = "TRACE")]
    Trace,
}

impl From<Level> for LogLevel {
    fn from(value: Level) -> Self {
        for l in LogLevel::iter() {
            if l.to_string() == value.as_str() {
                return l;
            }
        }
        LogLevel::Info
    }
}

impl Default for LogLevel {
    fn default() -> Self {
        Self::Info
    }
}

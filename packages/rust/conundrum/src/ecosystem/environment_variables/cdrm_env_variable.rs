use serde::{Deserialize, Serialize};
use winnow::error::ErrMode;

use crate::lang::runtime::state::conundrum_error_variant::{ConundrumErrorVariant, ConundrumModalResult};

pub static DEFAULT_CDRM_SERVER_PORT: &u32 = &3005;

#[typeshare::typeshare]
#[derive(Serialize, Deserialize, uniffi::Enum, strum_macros::Display, Clone, Debug)]
pub enum CdrmEnvVariable {
    #[serde(rename = "CDRM_LOG_LEVEL")]
    #[strum(to_string = "CDRM_LOG_LEVEL")]
    LogLevel,
    #[serde(rename = "CDRM_SERVER_PORT")]
    #[strum(to_string = "CDRM_SERVER_PORT")]
    ServerPort,
    #[serde(rename = "CDRM_CSL_FILE_PATH")]
    #[strum(to_string = "CDRM_CSL_FILE_PATH")]
    CSLFilePath,
}

impl CdrmEnvVariable {
    pub fn read(&self) -> ConundrumModalResult<String> {
        std::env::var(self.to_string()).map(|item| item.to_string()).map_err(|_| {
                                                                        ErrMode::Backtrack(
                   ConundrumErrorVariant::EnvVarNotFound(self.clone())
                )
                                                                    })
    }

    pub fn variant_default(&self) -> Option<String> {
        match self {
            Self::LogLevel => Some("WARN".to_string()),
            Self::ServerPort => Some(DEFAULT_CDRM_SERVER_PORT.to_string()),
            Self::CSLFilePath => None,
        }
    }

    /// Only use this in development!!
    #[allow(non_snake_case)]
    pub fn read_as_required_DEV_ONLY(&self, error_message: Option<String>) -> String {
        self.read().expect(error_message.unwrap_or_else(|| format!("The `{}`", self)).as_str())
    }
}

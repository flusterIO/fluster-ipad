use serde::{Deserialize, Serialize};
use winnow::error::ErrMode;

use crate::lang::runtime::state::conundrum_error_variant::{ConundrumErrorVariant, ConundrumModalResult};

#[typeshare::typeshare]
#[derive(Serialize, Deserialize, uniffi::Enum, strum_macros::Display, Clone, Debug)]
pub enum CdrmEnvVariable {
    #[serde(rename = "CDRM_LOG_LEVEL")]
    #[strum(to_string = "CDRM_LOG_LEVEL")]
    LogLevel,
}

impl CdrmEnvVariable {
    pub fn read(&self) -> ConundrumModalResult<String> {
        std::env::var(self.to_string()).map(|item| item.to_string()).map_err(|_| {
                                                                        ErrMode::Backtrack(
                   ConundrumErrorVariant::EnvVarNotFound(self.clone())
                )
                                                                    })
    }

    /// Only use this in development!!
    #[allow(non_snake_case)]
    pub fn read_as_required_DEV_ONLY(&self, error_message: Option<String>) -> String {
        self.read().expect(error_message.unwrap_or_else(|| format!("The `{}`", self)).as_str())
    }
}

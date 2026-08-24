use conundrum::lifted_models::primitives::date_time::DateTime;
use fake::Dummy;
use serde::{Deserialize, Serialize};

use crate::vector::models::ecosystem_data::{
    ecosystem_setting_key::{setting_key_trait::EcosystemSettingKey, unique_setting_key::UniqueSettingKey},
    ecosytem_setting_types::vector_generation_method::OptionalVectorGenerationMethod,
};

#[derive(Serialize, Deserialize, Clone, Debug, strum_macros::Display, Dummy, specta::Type)]
#[strum(serialize_all = "kebab-case")]
#[serde(rename_all = "kebab-case")]
pub enum StorageSettingKey {
    /// The number of days that logs should be saved. Defaults to 30.
    SaveLogDuration(u32),
    LogVectorGenMethod(OptionalVectorGenerationMethod),
}

impl EcosystemSettingKey for StorageSettingKey {
    fn to_setting_key(&self) -> UniqueSettingKey {
        match self {
            Self::SaveLogDuration(_) => UniqueSettingKey::SaveLogDuration,
            Self::LogVectorGenMethod(_) => UniqueSettingKey::LogVectorGenMethod,
        }
    }
}

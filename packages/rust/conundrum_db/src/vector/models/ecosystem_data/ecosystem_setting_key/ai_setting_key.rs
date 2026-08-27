use conundrum::ecosystem::error_handling::db_error::DatabaseError;
use fake::Dummy;
use serde::{Deserialize, Serialize};

use crate::vector::models::ecosystem_data::{
    ecosystem_setting_key::{setting_key_trait::EcosystemSettingKey, unique_setting_key::UniqueSettingKey},
    ecosytem_setting_types::vector_generation_method::OptionalVectorGenerationMethod,
};

#[derive(Serialize,
           Deserialize,
           Clone,
           Debug,
           strum_macros::Display,
           strum_macros::EnumIter,
           strum_macros::EnumString,
           specta::Type,
           Dummy)]
#[strum(serialize_all = "kebab-case")]
#[serde(rename_all = "kebab-case")]
pub enum AISettingKey {
    LocalAiPreference(f32),
    LogVectorGenMethod(OptionalVectorGenerationMethod),
    AutoCleanVectors(bool),
}

impl EcosystemSettingKey for AISettingKey {
    fn to_setting_key(&self) -> UniqueSettingKey {
        match self {
            Self::LocalAiPreference(_) => UniqueSettingKey::LocalAiPreference,
            Self::LogVectorGenMethod(_) => UniqueSettingKey::LogVectorGenMethod,
            Self::AutoCleanVectors(_) => UniqueSettingKey::AutoCleanVectors,
        }
    }
}

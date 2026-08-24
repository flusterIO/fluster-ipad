use fake::Dummy;
use serde::{Deserialize, Serialize};

use crate::vector::models::ecosystem_data::ecosystem_setting_key::{
    setting_key::Setting, setting_key_trait::EcosystemSettingKey, unique_setting_key::UniqueSettingKey,
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
}

impl EcosystemSettingKey for AISettingKey {
    fn to_setting_key(&self) -> UniqueSettingKey {
        match self {
            Self::LocalAiPreference(_) => UniqueSettingKey::LocalAiPreference,
        }
    }
}

use fake::Dummy;
use serde::{Deserialize, Serialize};

use crate::vector::models::ecosystem_data::ecosystem_setting_key::{
    setting_key_trait::EcosystemSettingKey, unique_setting_key::UniqueSettingKey,
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
pub enum PersonalizationSettingKey {
    FirstName(String),
    LastName(String),
    Profession(String),
}

impl EcosystemSettingKey for PersonalizationSettingKey {
    fn to_setting_key(&self) -> UniqueSettingKey {
        match self {
            Self::FirstName(_) => UniqueSettingKey::FirstName,
            Self::LastName(_) => UniqueSettingKey::LastName,
            Self::Profession(_) => UniqueSettingKey::Profession,
        }
    }
}

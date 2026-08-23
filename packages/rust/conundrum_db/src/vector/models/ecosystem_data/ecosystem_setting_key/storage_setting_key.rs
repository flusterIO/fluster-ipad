use conundrum::lifted_models::primitives::date_time::DateTime;
use fake::Dummy;
use serde::{Deserialize, Serialize};

use crate::vector::models::ecosystem_data::{
    ecosystem_setting::value_types::setting_value_type::SettingValueType,
    ecosystem_setting_key::setting_key_trait::EcosystemSettingKey,
    ecosytem_setting_types::vector_generation_method::OptionalVectorGenerationMethod,
};

#[derive(Serialize,
           Deserialize,
           Clone,
           Debug,
           strum_macros::Display,
           strum_macros::EnumIter,
           strum_macros::EnumString,
           Dummy)]
#[strum(serialize_all = "kebab-case")]
#[serde(rename_all = "kebab-case")]
pub enum StorageSettingKey {
    /// The number of days that logs should be saved. Defaults to 30.
    SaveLogDuration(u32),
    LogVectorGenMethod(OptionalVectorGenerationMethod),
}

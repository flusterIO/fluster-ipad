use fake::Dummy;
use serde::{Deserialize, Serialize};

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
pub enum UniqueSettingKey {
    AutoSyncOnNewChat,
    AutoSyncOnNewMsg,
    LocalAiPreference,
    SaveLogDuration,
    LogVectorGenMethod,
}


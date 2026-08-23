use fake::Dummy;
use serde::{Deserialize, Serialize};

use crate::vector::models::ecosystem_data::ecosystem_setting_key::setting_key_trait::EcosystemSettingKey;

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
pub enum SyncSettingKey {
    AutoSyncOnNewChat(bool),
    AutoSyncOnNewMsg(bool),
}

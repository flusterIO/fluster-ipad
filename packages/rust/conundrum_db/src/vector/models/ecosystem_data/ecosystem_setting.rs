use std::collections::HashMap;

use conundrum::ecosystem::ecosystem_setting::ecosystem_setting_key::EcosystemSettingKey;
use serde::{Deserialize, Serialize};

use crate::vector::models::ecosystem_data::ecosystem_setting_value::EcosystemSettingValue;

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct EcosystemSettings(HashMap<EcosystemSettingKey, EcosystemSettingValue>);

impl Default for EcosystemSettings {
    fn default() -> Self {
        Self(HashMap::new())
    }
}

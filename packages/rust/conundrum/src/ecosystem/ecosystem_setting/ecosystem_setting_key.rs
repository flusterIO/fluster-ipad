use serde::{Deserialize, Serialize};
use specta::Type;
use strum::{EnumIter, IntoEnumIterator};

use crate::lang::runtime::state::conundrum_error_variant::ConundrumErrorVariant;

#[derive(Serialize, Deserialize, Clone, Debug, strum_macros::Display, EnumIter, Type, PartialEq, Eq, Hash)]
pub enum EcosystemSettingKey {
    #[serde(rename = "first_name")]
    #[strum(to_string = "first_name")]
    FirstName,
    #[serde(rename = "last_name")]
    #[strum(to_string = "last_name")]
    LastName,
}

impl TryFrom<String> for EcosystemSettingKey {
    type Error = ConundrumErrorVariant;

    fn try_from(value: String) -> Result<Self, Self::Error> {
        for v in EcosystemSettingKey::iter() {
            if v.to_string() == value {
                return Ok(v);
            }
        }
        Err(ConundrumErrorVariant::InvalidSettingKey(value.clone()))
    }
}

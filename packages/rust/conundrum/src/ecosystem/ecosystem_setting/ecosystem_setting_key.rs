use arrow_schema::Field;
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
    #[serde(rename = "embedding_model")]
    #[strum(to_string = "embedding_model")]
    EmbeddingModel,
    #[strum(to_string = "notification-storage-method")]
    #[serde(rename = "notification-storage-method")]
    NotificationStorageMethod,
    #[strum(to_string = "store-logs-duration")]
    #[serde(rename = "store-logs-duration")]
    /// ## StoreLogsDuration
    ///
    /// Conundrum will periodically clean the log history to keep storage from
    /// growing indefinitely. This should be an integer representing the
    /// number of days to store logs.
    StoreLogsDuration,
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

impl EcosystemSettingKey {
    fn field_definition(field_key: &'static str, nullable: bool) -> Field {
        Field::new(field_key.to_string(), arrow_schema::DataType::Utf8, nullable)
    }
}

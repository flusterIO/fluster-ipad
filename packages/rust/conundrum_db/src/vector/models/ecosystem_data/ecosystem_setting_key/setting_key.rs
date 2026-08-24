use std::{fmt::Display, str::FromStr};

use conundrum::ecosystem::{
    db::db_traits::db_identifiable::DatabaseIdentifiable, error_handling::db_error::DatabaseError,
};
use fake::Dummy;
use serde::{Deserialize, Serialize};
use serde_with::{DeserializeFromStr, SerializeDisplay, serde_as};

use crate::vector::models::ecosystem_data::ecosystem_setting_key::{
    ai_setting_key::AISettingKey, storage_setting_key::StorageSettingKey, sync_setting_key::SyncSettingKey,
};

#[serde_as]
#[derive(Clone, Debug, Dummy, specta::Type, SerializeDisplay, DeserializeFromStr)]
#[serde(untagged)]
pub enum Setting {
    Sync(SyncSettingKey),
    AI(AISettingKey),
    Storage(StorageSettingKey),
}

impl Display for Setting {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let s = serde_json::to_string(&self).map_err(|e| {
                                                log::error!("Serialization Error: {:#?}", e);
                                                std::fmt::Error::default()
                                            })?;
        write!(f, "{}", s)
    }
}

impl FromStr for Setting {
    type Err = DatabaseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let x: Setting = serde_json::from_str(s).map_err(|e| {
                                                    log::error!("Error: {:#?}", e);
                                                    DatabaseError::SerializationError
                                                })?;
        Ok(x)
    }
}

impl DatabaseIdentifiable for Setting {
    fn to_predicate(&self, field_key: &str) -> String {
        format!("{} = \"{}\"", field_key, self.to_string())
    }
}

// impl SettingKey {
//     pub async fn read(&self, db: &ArcMutexDB) -> DatabaseResult<()> {
//         let x =
// EcosystemSettingModel::get_by_predicate(Some(self.to_predicate("key")),
//
// Some(PaginationParams::single()),
// None,                                                         db).await?;
//         let item = match x.len() {
//                        0 => {
//                            log::warn!("Setting not found for the `{}` key.",
// self.to_string());                            None
//                        }
//                        1 => Some(x.index(0)),
//                        _ => {
//                            log::warn!("Multiple settings not found for the
// `{}` key.", self.to_string());                            None
//                        }
//                    }.ok_or_else(||
// DatabaseError::InvalidSetting(self.to_string()))?;

//         Ok(())
//     }
// }

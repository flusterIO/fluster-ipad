use std::{fmt::Display, ops::Index, str::FromStr, sync::Arc};

use conundrum::ecosystem::{
    db::{
        db::ArcMutexDB,
        db_traits::{db_identifiable::DatabaseIdentifiable, entity_crud::EntityCRUD},
        parameters::general::pagination::PaginationParams,
    },
    error_handling::db_error::{DatabaseError, DatabaseResult},
};
use fake::Dummy;
use serde::{Deserialize, Serialize};
use serde_with::{DeserializeFromStr, SerializeDisplay, serde_as};

use crate::vector::models::ecosystem_data::{
    ecosystem_setting_key::{
        ai_setting_key::AISettingKey, setting_key_trait::EcosystemSettingKey, storage_setting_key::StorageSettingKey,
        sync_setting_key::SyncSettingKey,
    },
    ecosytem_setting_types::ecosystem_setting_model::EcosystemSettingModel,
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

impl EcosystemSettingKey for Setting {
    fn to_setting_key(&self) -> super::unique_setting_key::UniqueSettingKey {
        match self {
            Self::Sync(s) => s.to_setting_key(),
            Self::AI(a) => a.to_setting_key(),
            Self::Storage(s) => s.to_setting_key(),
        }
    }
}

impl Setting {
    pub fn to_model(&self) -> EcosystemSettingModel {
        let key = self.to_setting_key();
        EcosystemSettingModel { key,
                                data: self.clone() }
    }

    pub async fn save(&self, db: &ArcMutexDB) -> DatabaseResult<()> {
        let model = self.to_model();
        EcosystemSettingModel::save_many(vec![model], &Arc::clone(db)).await?;
        Ok(())
    }

    pub async fn read(&self, db: &ArcMutexDB) -> DatabaseResult<Self> {
        let x = EcosystemSettingModel::get_by_predicate(Some(self.to_predicate("key")),
                                                        Some(PaginationParams::single()),
                                                        None,
                                                        db).await?;
        let item = match x.len() {
                       0 => {
                           log::warn!("Setting not found for the `{}` key.", self);
                           None
                       }
                       1 => Some(x.index(0)),
                       _ => {
                           log::warn!(
                                      "Multiple settings not found for the
`{}` key.",
                                      self
            );
                           None
                       }
                   }.ok_or_else(|| DatabaseError::InvalidSetting(self.to_string()))?;

        Ok(item.data.clone())
    }
}

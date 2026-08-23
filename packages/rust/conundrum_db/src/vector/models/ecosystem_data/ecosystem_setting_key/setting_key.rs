use std::ops::Index;

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

use crate::vector::models::{
    academic::result::academic_result_metric::AcademicResultMetric,
    ecosystem_data::ecosystem_setting_key::{
        ai_setting_key::AISettingKey, storage_setting_key::StorageSettingKey, sync_setting_key::SyncSettingKey,
    },
};

#[derive(Serialize, Deserialize, Clone, Debug, strum_macros::Display, Dummy)]
#[serde(untagged)]
pub enum SettingKey {
    Sync(SyncSettingKey),
    AI(AISettingKey),
    Storage(StorageSettingKey),
}

impl DatabaseIdentifiable for SettingKey {
    fn to_predicate(&self, field_key: &str) -> String {
        format!("{} = \"{}\"", field_key, self.to_string())
    }
}

impl SettingKey {
    pub async fn read(&self, db: &ArcMutexDB) -> DatabaseResult<()> {
        let x = EcosystemSettingModel::get_by_predicate(Some(self.to_predicate("key")),
                                                        Some(PaginationParams::single()),
                                                        None,
                                                        db).await?;
        let item = match x.len() {
                       0 => {
                           log::warn!("Setting not found for the `{}` key.", self.to_string());
                           None
                       }
                       1 => Some(x.index(0)),
                       _ => {
                           log::warn!("Multiple settings not found for the `{}` key.", self.to_string());
                           None
                       }
                   }.ok_or_else(|| DatabaseError::InvalidSetting(self.to_string()))?;

        Ok(())
    }
}

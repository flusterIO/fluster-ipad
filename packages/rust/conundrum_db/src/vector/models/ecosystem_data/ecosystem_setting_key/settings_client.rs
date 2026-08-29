use std::sync::Arc;

use conundrum::ecosystem::{
    db::db::ArcMutexDB,
    error_handling::db_error::{DatabaseError, DatabaseResult},
};
use lancedb::database;

use crate::vector::models::ecosystem_data::{
    ecosystem_setting_key::{
        ai_setting_key::AISettingKey, setting_key::Setting, sync_setting_key::SyncSettingKey,
        unique_setting_key::UniqueSettingKey,
    },
    ecosytem_setting_types::ecosystem_setting_model::EcosystemSettingModel,
};

/// All methods fall back to the default value. If they throw an error, it's
/// because the ecosystem data is damaged or somebody really f--ked up.
pub struct SettingsClient;

impl SettingsClient {
    async fn get_setting(setting_key: UniqueSettingKey, db: ArcMutexDB) -> EcosystemSettingModel {
        if let Ok(res) = EcosystemSettingModel::get_by_setting_key(setting_key.clone(), Arc::clone(&db)).await {
            match res {
                Some(s) => s,
                None => {
                    let model: EcosystemSettingModel = setting_key.into();
                    model
                }
            }
        } else {
            let model: EcosystemSettingModel = setting_key.into();
            model
        }
    }

    pub async fn should_clean_vectors(database: ArcMutexDB) -> DatabaseResult<bool> {
        let setting = SettingsClient::get_setting(UniqueSettingKey::AutoCleanVectors, Arc::clone(&database)).await;
        match setting.data {
            Setting::AI(k) => match k {
                AISettingKey::AutoCleanVectors(b) => Ok(b),
                _ => Err(DatabaseError::InvalidSetting(UniqueSettingKey::AutoCleanVectors.to_string())),
            },
            _ => Err(DatabaseError::InvalidSetting(UniqueSettingKey::AutoCleanVectors.to_string())),
        }
    }

    pub async fn max_sync_threads(database: ArcMutexDB) -> DatabaseResult<u16> {
        let setting = SettingsClient::get_setting(UniqueSettingKey::MaxSyncThreads, Arc::clone(&database)).await;
        match setting.data {
            Setting::Sync(k) => match k {
                SyncSettingKey::MaxSyncThreads(b) => Ok(b),
                _ => Err(DatabaseError::InvalidSetting(UniqueSettingKey::MaxSyncThreads.to_string())),
            },
            _ => Err(DatabaseError::InvalidSetting(UniqueSettingKey::MaxSyncThreads.to_string())),
        }
    }
}

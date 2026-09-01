use std::sync::Arc;

use conundrum::ecosystem::db::{
    db_traits::{db_entity::DBSchema, entity_crud::EntityCRUD},
    tables::DatabaseTable,
};
use strum::IntoEnumIterator;

use crate::vector::{
    models::ecosystem_data::{
        ecosystem_setting_key::{setting_key::Setting, unique_setting_key::UniqueSettingKey},
        ecosytem_setting_types::ecosystem_setting_model::EcosystemSettingModel,
    },
    seed::seed_content::{SeedChunks, SeedContent},
};

pub struct SettingsSeeder {}

impl SeedContent for SettingsSeeder {
    async fn try_seed<'a>(&self,
                          db: conundrum::ecosystem::db::db::ArcMutexDB)
                          -> conundrum::ecosystem::error_handling::db_error::DatabaseResult<()> {
        for setting_key in UniqueSettingKey::iter() {
            let default_setting = Setting::from(setting_key);
            let model = default_setting.to_model();
            <EcosystemSettingModel as EntityCRUD< <EcosystemSettingModel as DBSchema>::PartialUpdateType>>::merge_by_primary_key(vec![model], Arc::clone(&db)).await?;
        }
        Ok(())
    }
}

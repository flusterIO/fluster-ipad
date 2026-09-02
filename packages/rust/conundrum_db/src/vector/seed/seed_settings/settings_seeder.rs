use std::sync::Arc;

use conundrum::ecosystem::db::{
    db_client::db_client::DBClient,
    db_traits::{db_entity::DBSchema, entity_crud::EntityCRUD, into_partial::IntoPartial},
    tables::DatabaseTable,
};
use strum::IntoEnumIterator;

use crate::vector::{
    models::ecosystem_data::{
        ecosystem_setting_key::{setting_key::Setting, unique_setting_key::UniqueSettingKey},
        ecosytem_setting_types::{
            ecosystem_setting_model::EcosystemSettingModel, stringified_setting::EcosystemSettingEntity,
        },
    },
    seed::seed_content::{SeedChunks, SeedContent},
};

pub struct SettingsSeeder {}

impl Default for SettingsSeeder {
    fn default() -> Self {
        Self {}
    }
}

impl SeedContent for SettingsSeeder {
    async fn try_seed<'a>(&self, db: DBClient) -> conundrum::ecosystem::error_handling::db_error::DatabaseResult<()> {
        let mut models: Vec<<EcosystemSettingEntity as DBSchema>::PartialUpdateType> =
            Vec::<<EcosystemSettingEntity as DBSchema>::PartialUpdateType>::new();
        for setting_key in UniqueSettingKey::iter() {
            log::debug!("Seeding Setting: {:?}", setting_key);
            // I'm not proud of myself, but it works...
            let default_setting = Setting::from(setting_key);
            let model = default_setting.to_model();
            let entity = model.to_entity()?;
            let partial: <EcosystemSettingEntity as DBSchema>::PartialUpdateType = entity.into_partial();
            models.push(partial);
        }
        <EcosystemSettingEntity as EntityCRUD<<EcosystemSettingEntity as
        DBSchema>::PartialUpdateType>>::merge_by_primary_key(models,
        db.clone()).await?;
        Ok(())
    }
}

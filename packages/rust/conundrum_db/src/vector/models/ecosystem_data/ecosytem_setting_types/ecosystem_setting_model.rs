use arrow_array::{RecordBatch, StringArray};
use conundrum::{
    ecosystem::{
        db::{
            db_client::db_client::DBClient,
            db_traits::{
                db_entity::{ArrowFields, DBEntity, DBSchema},
                db_field::DatabaseField,
                entity_crud::EntityCRUD,
            },
            tables::DatabaseTable,
        },
        error_handling::db_error::{DatabaseError, DatabaseResult},
    },
    impl_default_crud,
    lang::lib::std_lib_impls::json_string::QuotedString,
};
use conundrum_macros::{DBDefaultCrud, DBPartial};
use serde::{Deserialize, Serialize};
use std::sync::Arc;

use crate::vector::models::ecosystem_data::{
    ecosystem_setting_key::{setting_key::Setting, unique_setting_key::UniqueSettingKey},
    ecosytem_setting_types::stringified_setting::EcosystemSettingEntity,
};

/// Warning: Don't use this directly. Use the enum to handle all interactions
/// with the DB for typesafetey.
#[derive(Serialize, Deserialize, Clone, Debug, specta::Type, fake::Dummy)]
pub struct EcosystemSettingModel {
    pub key: UniqueSettingKey,
    pub data: Setting,
}

impl TryFrom<EcosystemSettingEntity> for EcosystemSettingModel {
    type Error = DatabaseError;

    fn try_from(value: EcosystemSettingEntity) -> Result<Self, Self::Error> {
        let data: Setting = serde_json::from_str(&value.data).map_err(|e| {
                                                                 log::error!("Setting Serialization Error: {:#?}", e);
                                                                 DatabaseError::SerializationError
                                                             })?;
        Ok(EcosystemSettingModel { key: value.key.clone(),
                                   data })
    }
}

impl From<UniqueSettingKey> for EcosystemSettingModel {
    fn from(value: UniqueSettingKey) -> Self {
        EcosystemSettingModel { key: value.clone(),
                                data: Setting::from(value) }
    }
}

impl EcosystemSettingModel {
    pub async fn get_by_setting_key(key: UniqueSettingKey, db: DBClient) -> DatabaseResult<Option<Self>> {
        let entity = EcosystemSettingEntity::get_by_setting_key(key, db).await?;
        let res = match entity {
            Some(en) => {
                let model = EcosystemSettingModel::try_from(en)?;
                Some(model)
            }
            None => None,
        };
        Ok(res)
    }

    pub async fn save(&self, db: DBClient) -> DatabaseResult<()> {
        let entity = self.to_entity()?;
        entity.save(db).await
    }

    pub fn to_entity(&self) -> DatabaseResult<EcosystemSettingEntity> {
        EcosystemSettingEntity::try_from(self.clone())
    }
}

use arrow_array::{RecordBatch, StringArray};
use conundrum::{
    ecosystem::{
        db::{
            db_client::db_client::DBClient,
            db_traits::{
                db_entity::{ArrowFields, DBEntity, DBSchema},
                db_field::DatabaseField,
                entity_crud::EntityCRUD,
                into_partial::IntoPartial,
            },
            tables::DatabaseTable,
        },
        error_handling::db_error::{DatabaseError, DatabaseResult},
    },
    impl_default_crud,
    lang::lib::std_lib_impls::json_string::QuotedString,
};
use conundrum_macros::{DBDefaultCrud, DBPartial, DatabaseEntity};
use serde::{Deserialize, Serialize};
use std::sync::Arc;

use crate::vector::models::ecosystem_data::{
    ecosystem_setting_key::{setting_key::Setting, unique_setting_key::UniqueSettingKey},
    ecosytem_setting_types::ecosystem_setting_model::EcosystemSettingModel,
};

/// Warning: Don't use this directly. Use the enum to handle all interactions
/// with the DB for typesafetey.
#[derive(Serialize, Deserialize, Clone, Debug, specta::Type, fake::Dummy, DatabaseEntity)]
#[db(table = DatabaseTable::EcosystemSetting)]
pub struct EcosystemSettingEntity {
    #[db(primary)]
    pub key: UniqueSettingKey,
    #[db(partial(required))]
    pub data: String,
}

impl IntoPartial<EcosystemSettingEntityPartial> for EcosystemSettingEntity {
    fn into_partial(&self) -> EcosystemSettingEntityPartial {
        EcosystemSettingEntityPartial { key: self.key.clone(),
                                        data: self.data.clone() }
    }
}

impl TryFrom<EcosystemSettingModel> for EcosystemSettingEntity {
    type Error = DatabaseError;

    fn try_from(value: EcosystemSettingModel) -> Result<Self, Self::Error> {
        let data = serde_json::to_string(&value.data).map_err(|e| {
                                                         log::error!("Setting Serialization Error: {:#?}", e);
                                                         DatabaseError::SerializationError
                                                     })?;
        Ok(EcosystemSettingEntity { key: value.key.clone(),
                                    data })
    }
}

impl EcosystemSettingEntity {
    pub async fn get_by_setting_key(key: UniqueSettingKey, db: DBClient) -> DatabaseResult<Option<Self>> {
        <EcosystemSettingEntity as EntityCRUD< <EcosystemSettingEntity as DBSchema>::PartialUpdateType>>::get_one_by_predicate(Some(format!("key = {}",
                                                                                 key.to_string().to_quoted_string()?)),
                                                                    None,
                                                                    db.clone()).await
    }

    pub async fn save(&self, db: DBClient) -> DatabaseResult<()> {
        <EcosystemSettingEntity as EntityCRUD< <EcosystemSettingEntity as DBSchema>::PartialUpdateType>>::merge_by_primary_key(vec![self.into_partial()], db.clone()).await
    }
}

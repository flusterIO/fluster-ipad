use arrow_array::{RecordBatch, StringArray};
use conundrum::{
    ecosystem::{
        db::{
            db::ArcMutexDB,
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

use crate::vector::models::ecosystem_data::ecosystem_setting_key::{
    setting_key::Setting, unique_setting_key::UniqueSettingKey,
};

/// Warning: Don't use this directly. Use the enum to handle all interactions
/// with the DB for typesafetey.
#[derive(Serialize, Deserialize, Clone, Debug, specta::Type, fake::Dummy, DBDefaultCrud, DBPartial)]
#[db(table = DatabaseTable::EcosystemSetting)]
pub struct EcosystemSettingModel {
    #[db(primary)]
    pub key: UniqueSettingKey,
    pub data: Setting,
}

impl From<UniqueSettingKey> for EcosystemSettingModel {
    fn from(value: UniqueSettingKey) -> Self {
        EcosystemSettingModel { key: value.clone(),
                                data: Setting::from(value) }
    }
}

impl EcosystemSettingModel {
    pub async fn get_by_setting_key(key: UniqueSettingKey, db: ArcMutexDB) -> DatabaseResult<Option<Self>> {
        EcosystemSettingModel::get_one_by_predicate(Some(format!("key = {}", key.to_string().to_quoted_string()?)),
                                                    None,
                                                    Arc::clone(&db)).await
    }

    pub async fn save(&self, db: ArcMutexDB) -> DatabaseResult<()> {
        EcosystemSettingModel::merge_by_primary_key(vec![self.clone()], Arc::clone(&db)).await
    }
}

impl<'a> ArrowFields<'a> for EcosystemSettingModel {
    fn arrow_fields(
        )
        -> conundrum::ecosystem::error_handling::db_error::DatabaseResult<Vec<std::sync::Arc<arrow_schema::Field>>>
    {
        Ok(vec![Arc::new(String::field_definition("key", false)), Arc::new(String::field_definition("key", false)),])
    }
}

impl<'a> DBEntity for EcosystemSettingModel {
    type IDType = UniqueSettingKey;
    type PartialUpdateType = EcosystemSettingModel;

    fn table() -> conundrum::ecosystem::db::tables::DatabaseTable {
        DatabaseTable::EcosystemSetting
    }

    fn merge_keys() -> &'static [&'static str] {
        &["key"]
    }

    fn primary_key() -> &'static str {
        "key"
    }

    fn set_primary_value(&mut self, value: UniqueSettingKey) {
        self.key = value.clone();
    }

    fn primary_value(&self) -> UniqueSettingKey {
        self.key.clone()
    }

    fn get_record_batch(data: Vec<Self>)
                        -> conundrum::ecosystem::error_handling::db_error::DatabaseResult<arrow_array::RecordBatch>
        where Self: Sized + Clone + Serialize {
        let schema = Self::schema()?;
        let mut keys = Vec::new();
        let mut datas = Vec::new();
        for item in data {
            keys.push(item.key.to_string());
            let s = serde_json::to_string(&item.data).map_err(|e| {
                                                         log::error!("Serialization Error: {:#?}", e);
                                                         DatabaseError::SerializationError
                                                     })?;
            datas.push(s);
        }
        let batch =
            RecordBatch::try_new(Arc::new(schema),
                                 vec![Arc::new(StringArray::from(keys)), Arc::new(StringArray::from(datas)),]).unwrap();
        Ok(batch)
    }
}

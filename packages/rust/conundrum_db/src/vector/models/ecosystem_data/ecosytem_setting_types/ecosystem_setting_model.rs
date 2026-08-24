use conundrum::ecosystem::db::db_traits::{db_entity::DBSchema, db_field::DatabaseField};
use serde::{Deserialize, Serialize};
use std::sync::Arc;

use crate::vector::models::ecosystem_data::ecosystem_setting_key::{
    setting_key::Setting, unique_setting_key::UniqueSettingKey,
};

#[derive(Serialize, Deserialize, Clone, Debug, specta::Type, fake::Dummy)]
pub struct EcosystemSettingModel {
    pub key: UniqueSettingKey,
    pub data: Setting,
}

impl<'a> DBSchema<'a> for EcosystemSettingModel {
    fn arrow_fields(
        )
        -> conundrum::ecosystem::error_handling::db_error::DatabaseResult<Vec<std::sync::Arc<arrow_schema::Field>>>
    {
        Ok(vec![Arc::new(String::field_definition("key", false)), Arc::new(String::field_definition("key", false)),])
    }
}

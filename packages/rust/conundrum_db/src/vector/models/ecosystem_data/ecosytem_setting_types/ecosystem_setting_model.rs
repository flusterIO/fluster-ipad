use conundrum::ecosystem::db::db_traits::db_entity::DBSchema;
use serde::{Deserialize, Serialize};

use crate::vector::models::ecosystem_data::ecosystem_setting_key::setting_key::SettingKey;

#[derive(Serialize, Deserialize, Clone, Debug, specta::Type, fake::Dummy)]
pub struct EcosystemSettingModel(SettingKey);

impl<'a> DBSchema<'a> for EcosystemSettingModel {
    fn arrow_fields(
        )
        -> conundrum::ecosystem::error_handling::db_error::DatabaseResult<Vec<std::sync::Arc<arrow_schema::Field>>>
    {
        let opts = Self::schema_options()?;
        Vec::<arrow_schema::FieldRef>::from_type::<Self>(opts).map_err(|e| {
            log::error!("Arrow Fields Error: {:?}", e);
            conundrum::ecosystem::error_handling::db_error::DatabaseError::SerializationError
        })
    }
}

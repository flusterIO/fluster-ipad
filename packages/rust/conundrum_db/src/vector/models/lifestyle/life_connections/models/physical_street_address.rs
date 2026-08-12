use std::sync::Arc;

use conundrum::ecosystem::db::traits::db_entity::DBSchema;
use fake::Dummy;

use crate::vector::{database::db_traits::db_field::DatabaseField, models::primitives::db_id::DatabaseId};

#[derive(serde::Serialize, serde::Deserialize, Clone, Debug, specta::Type, Dummy)]
pub struct PhysicalStreetAddress {
    pub id: DatabaseId,
    /// Example: `123 E Main St`
    pub street_address: Option<String>,
    pub country: Option<String>,
    pub zip: Option<String>,
}

impl<'a> DBSchema<'a> for PhysicalStreetAddress {
    fn arrow_fields(
        )
        -> conundrum::ecosystem::error_handling::db_error::DatabaseResult<Vec<std::sync::Arc<arrow_schema::Field>>>
    {
        Ok(vec![Arc::new(DatabaseId::field_definition("id", false)),
                Arc::new(String::field_definition("street_address", true)),
                Arc::new(String::field_definition("country", true)),
                Arc::new(String::field_definition("zip", true)),])
    }
}

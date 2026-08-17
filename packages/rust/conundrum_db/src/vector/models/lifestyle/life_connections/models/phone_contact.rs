use std::sync::Arc;

use arrow_schema::{DataType, Field, Fields};
use conundrum::ecosystem::db::db_traits::{db_entity::DBSchema, db_field::DatabaseField};
use fake::Dummy;

use crate::vector::models::lifestyle::life_connections::models::phone_number_type::PhoneNumberType;

#[derive(serde::Serialize, serde::Deserialize, Clone, Debug, specta::Type, Dummy)]
pub struct PhoneContact {
    /// The phone number, stored as a string.
    pub number: String,
    pub phone_type: Option<PhoneNumberType>,
}

impl<'a> DBSchema<'a> for PhoneContact {
    fn arrow_fields() -> conundrum::ecosystem::error_handling::db_error::DatabaseResult<Vec<std::sync::Arc<Field>>> {
        Ok(vec![Arc::new(String::field_definition("number", false)),
                Arc::new(String::field_definition("phone_type", true))])
    }
}

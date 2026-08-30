use std::sync::Arc;

use arrow_schema::{DataType, Field, Fields};
use conundrum::ecosystem::db::{
    db_traits::{db_entity::DBSchema, db_field::DatabaseField},
    tables::DatabaseTable,
};
use conundrum_macros::DatabaseEntity;
use fake::Dummy;

use crate::vector::models::lifestyle::life_connections::models::phone_number_type::PhoneNumberType;

#[derive(serde::Serialize, serde::Deserialize, Clone, Debug, specta::Type, Dummy, DatabaseEntity)]
#[db(table = DatabaseTable::PhoneContact)]
pub struct PhoneContact {
    /// The phone number, stored as a string.
    pub number: String,
    pub phone_type: Option<PhoneNumberType>,
}

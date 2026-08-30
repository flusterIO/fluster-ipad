use std::sync::Arc;

use conundrum::{
    ecosystem::db::{
        db_traits::{db_entity::DBSchema, db_field::DatabaseField},
        tables::DatabaseTable,
    },
    lifted_models::primitives::db_id::DatabaseId,
};
use conundrum_macros::DatabaseEntity;
use fake::Dummy;

#[derive(serde::Serialize, serde::Deserialize, Clone, Debug, specta::Type, Dummy, DatabaseEntity)]
#[db(table = DatabaseTable::StreetAddress)]
pub struct PhysicalStreetAddress {
    pub id: DatabaseId,
    /// Example: `123 E Main St`
    pub street_address: Option<String>,
    pub country: Option<String>,
    pub zip: Option<String>,
}

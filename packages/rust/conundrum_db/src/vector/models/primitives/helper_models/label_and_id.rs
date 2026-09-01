use std::sync::Arc;

use conundrum::{
    ecosystem::db::db_traits::{db_entity::DBSchema, db_field::DatabaseField},
    lifted_models::primitives::db_id::DatabaseId,
};
use conundrum_macros::{DBPartial, DBSchema, DatabaseEntity};

#[derive(serde::Serialize, serde::Deserialize, Clone, Debug, specta::Type, fake::Dummy, DBSchema, DBPartial)]
pub struct IDAndLabel {
    pub id: DatabaseId,
    pub label: Option<String>,
}

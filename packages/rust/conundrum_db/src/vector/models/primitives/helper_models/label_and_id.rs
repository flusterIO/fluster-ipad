use std::sync::Arc;

use conundrum::{
    ecosystem::db::db_traits::{db_entity::DBSchema, db_field::DatabaseField},
    lifted_models::primitives::db_id::DatabaseId,
};
use conundrum_macros::{DBSchema, DatabaseEntity};

#[derive(serde::Serialize, serde::Deserialize, Clone, Debug, specta::Type, fake::Dummy, DBSchema)]
pub struct IDAndOptionalLabel {
    pub id: DatabaseId,
    pub label: Option<String>,
}

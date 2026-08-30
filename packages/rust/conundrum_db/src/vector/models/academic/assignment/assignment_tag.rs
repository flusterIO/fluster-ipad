use std::sync::Arc;

use conundrum::{
    ecosystem::db::{
        db_traits::{db_entity::DBSchema, db_field::DatabaseField},
        tables::DatabaseTable,
    },
    lifted_models::primitives::db_id::DatabaseId,
};
use conundrum_macros::DatabaseEntity;

#[derive(serde::Serialize, serde::Deserialize, Clone, Debug, specta::Type, fake::Dummy, DatabaseEntity)]
#[db(table = DatabaseTable::AssignmentTag)]
pub struct AssignmentTag {
    pub tag_value: String,
    pub assignment_id: DatabaseId,
}

use std::sync::Arc;

use conundrum::{
    ecosystem::db::{
        db_traits::{db_entity::DBSchema, db_field::DatabaseField, db_identifiable::DatabaseIdentifiable},
        tables::DatabaseTable,
    },
    lifted_models::primitives::db_id::DatabaseId,
};
use conundrum_macros::DatabaseEntity;

#[derive(serde::Serialize, serde::Deserialize, Clone, Debug, specta::Type, fake::Dummy, DatabaseEntity)]
#[db(table = DatabaseTable::AssignmentSubject)]
pub struct AssignmentSubject {
    pub subject_value: String,
    pub subject_id: DatabaseId,
}

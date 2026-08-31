use std::sync::Arc;

use conundrum::{
    ecosystem::db::{
        db_traits::{
            db_entity::{DBEntity, DBSchema},
            db_field::DatabaseField,
        },
        tables::DatabaseTable,
    },
    impl_default_crud,
    lifted_models::primitives::{date_time::DateTime, db_id::DatabaseId},
};
use conundrum_macros::DatabaseEntity;
use fake::Dummy;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone, Debug, specta::Type, Dummy, DatabaseEntity)]
#[db(table = DatabaseTable::Assignment)]
pub struct AssignmentEntity {
    pub id: DatabaseId,
    pub label: String,
    pub description: Option<String>,
    pub due_at: Option<DateTime>,
    pub ctime: DateTime,
    pub utime: DateTime,
}

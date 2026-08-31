use conundrum::{
    ecosystem::db::{
        db_traits::{db_entity::DBSchema, db_field::DatabaseField},
        tables::DatabaseTable,
    },
    lifted_models::primitives::db_id::DatabaseId,
};
use conundrum_macros::DatabaseEntity;

use crate::topic_join;
use std::sync::Arc;

#[derive(serde::Serialize, serde::Deserialize, Clone, Debug, specta::Type, fake::Dummy, DatabaseEntity)]
#[db(table = DatabaseTable::AssigmentTopic)]
pub struct AssignmentTopic {
    pub id: DatabaseId,
    pub topic_value: String,
    pub topic_id: DatabaseId,
}

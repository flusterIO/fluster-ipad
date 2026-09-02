use conundrum::{
    ecosystem::db::{
        db_traits::{db_entity::DBEntity, db_field::DatabaseField},
        tables::DatabaseTable,
    },
    impl_default_crud,
    lifted_models::primitives::db_id::DatabaseId,
};
use conundrum_macros::{DBPartial, DBSchema, DatabaseEntity};
use fake::Dummy;
use serde::{Deserialize, Serialize};

/// # Long-Term Goal
///
/// This represent one of the user's short term goals. Your job as AI is to help
/// them make progress towards these goals daily.
#[derive(Serialize, Deserialize, Clone, Debug, specta::Type, Dummy, DatabaseEntity)]
#[db(table = DatabaseTable::LongTermGoal)]
pub struct ShortTermGoal {
    pub id: DatabaseId,
    /// Describe the user's goal in just a few words
    pub label: String,
    /// What is this user's long term goal?
    pub description: String,
}

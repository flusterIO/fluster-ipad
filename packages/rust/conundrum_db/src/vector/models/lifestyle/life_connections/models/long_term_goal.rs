use conundrum::{
    ecosystem::db::db_traits::{db_entity::DBEntity, db_field::DatabaseField},
    impl_default_crud,
    lifted_models::primitives::db_id::DatabaseId,
};
use conundrum_macros::{DBPartial, DBSchema};
use fake::Dummy;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone, Debug, specta::Type, Dummy)]
pub struct LongTermGoal {
    pub id: DatabaseId,
    /// Describe the user's goal in just a few words
    pub label: String,
    /// What is this user's long term goal?
    pub description: String,
}

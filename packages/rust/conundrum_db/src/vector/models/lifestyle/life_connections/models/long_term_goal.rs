use conundrum::{
    ecosystem::db::db_traits::{db_entity::DBEntity, db_field::DatabaseField},
    impl_default_crud,
    lifted_models::primitives::db_id::DatabaseId,
};
use conundrum_macros::{DBPartial, DBSchema};
use fake::Dummy;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone, Debug, DBSchema, DBPartial, specta::Type, Dummy)]
pub struct LongTermGoal {
    #[db(partial(required))]
    pub id: DatabaseId,
    /// Describe the user's goal in just a few words
    pub label: String,
    /// What is this user's long term goal?
    pub description: String,
}

impl_default_crud!(LongTermGoal, LongTermGoal, DatabaseId);

impl<'a> DBEntity<'a, DatabaseId> for LongTermGoal {
    type PartialUpdateType = LongTermGoal;

    fn table() -> conundrum::ecosystem::db::tables::DatabaseTable {
        conundrum::ecosystem::db::tables::DatabaseTable::LongTermGoal
    }

    fn merge_keys() -> &'static [&'static str] {
        &["id"]
    }

    fn primary_key() -> &'static str {
        "id"
    }

    fn primary_value(&self) -> DatabaseId {
        self.id.clone()
    }

    fn set_primary_value(&mut self, value: DatabaseId) {
        self.id = value.clone();
    }
}

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
    lifted_models::primitives::db_id::DatabaseId,
};
use conundrum_macros::DatabaseEntity;
use fake::Dummy;

#[derive(serde::Serialize, serde::Deserialize, Clone, Debug, specta::Type, Dummy, DatabaseEntity)]
#[db(table = DatabaseTable::UserWorkspaceRepository)]
pub struct UserWorkspaceRepository {
    pub workspace_root: String,
    pub repository_id: DatabaseId,
}

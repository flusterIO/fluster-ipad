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

#[derive(serde::Serialize, serde::Deserialize, Clone, Debug, specta::Type)]
pub struct UserWorkspaceRepository {
    pub workspace_root: String,
    pub repository_id: DatabaseId,
}

impl<'a> DBSchema<'a> for UserWorkspaceRepository {
    fn arrow_fields(
        )
        -> conundrum::ecosystem::error_handling::db_error::DatabaseResult<Vec<std::sync::Arc<arrow_schema::Field>>>
    {
        Ok(vec![Arc::new(String::field_definition("workspace_root", false)),
                Arc::new(DatabaseId::field_definition("repository_id", false))])
    }
}

impl_default_crud!(UserWorkspaceRepository, UserWorkspaceRepository, String);

impl<'a> DBEntity<'a> for UserWorkspaceRepository {
    type PartialUpdateType;

    fn table() -> conundrum::ecosystem::db::tables::DatabaseTable {
        DatabaseTable::WorkspaceRepository
    }

    fn merge_keys() -> &'static [&'static str] {
        &["workspace_root", "repository_id"]
    }

    fn primary_key() -> &'static str {
        "workspace_root"
    }

    fn primary_value(&self) -> String {
        self.workspace_root.clone()
    }

    fn set_primary_value(&mut self, value: String) {
        self.workspace_root = value.clone();
    }
}

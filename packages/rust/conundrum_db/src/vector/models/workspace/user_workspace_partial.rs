use std::sync::Arc;

use arrow_schema::{DataType, Field};
use conundrum::ecosystem::db::traits::db_entity::DBSchema;
use fake::Dummy;
use serde::{Deserialize, Serialize};
use specta::Type;

use crate::vector::{
    database::db_traits::db_field::DatabaseField,
    models::{ai::ai_interactions::AIInteractions, workspace::user_workspace::UserWorkspace},
};

#[derive(Serialize, Deserialize, Clone, Debug, Type, Dummy)]
pub struct UserWorkspacePartial {
    /// The path to the root of the workspace and the primary key for the
    /// workspace. This is still required to update the proper item.
    pub root: String,
    pub label: Option<String>,
    pub respect_gitignore: Option<bool>,
    pub ignore_hidden: Option<bool>,
    pub resource_dir: Option<String>,
    pub ai: Option<AIInteractions>,
}

impl<'a> DBSchema<'a> for UserWorkspacePartial {
    fn arrow_fields() -> conundrum::ecosystem::error_handling::db_error::DatabaseResult<Vec<Arc<Field>>> {
        Ok(vec![Arc::new(String::field_definition("root", false)),
                Arc::new(String::field_definition("label", true)),
                Arc::new(bool::field_definition("respect_gitignore", true)),
                Arc::new(bool::field_definition("ignore_hidden", true)),
                Arc::new(String::field_definition("resource_dir", true)),
                Arc::new(AIInteractions::field_definition("ai", true))])
    }
}

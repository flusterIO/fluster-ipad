use std::sync::Arc;

use arrow_schema::{DataType, Field};
use fake::Dummy;
use serde::{Deserialize, Serialize};
use specta::Type;

use crate::vector::{
    database::db_traits::db_entity::ArrowSchemaRepresentable, models::workspace::user_workspace::UserWorkspace,
};

#[derive(Serialize, Deserialize, Clone, Debug, Type, Dummy)]
pub struct UserWorkspacePartial {
    /// The path to the root of the workspace and the primary key for the
    /// workspace. This is still required to update the proper item.
    pub root: String,
    pub label: Option<String>,
    pub bib_paths: Option<Vec<String>>,
    pub respect_gitignore: Option<bool>,
    pub ignore_hidden: Option<bool>,
    pub resource_dir: Option<String>,
}

impl ArrowSchemaRepresentable for UserWorkspacePartial {
    fn arrow_schema() -> std::sync::Arc<arrow_schema::Schema> {
        Arc::new(arrow_schema::Schema::new(vec![Field::new("root", DataType::Utf8, false),
                                                Field::new("label", DataType::Utf8, true),
                                                Field::new("bib_paths",
                                                           DataType::List(Arc::new(UserWorkspace::item_field_def())),
                                                           true),
                                                Field::new("respect_gitignore", DataType::Boolean, true),
                                                Field::new("ignore_hidden", DataType::Boolean, true),
                                                Field::new("resource_dir", DataType::Utf8, true),]))
    }
}

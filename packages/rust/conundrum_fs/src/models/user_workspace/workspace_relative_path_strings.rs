use arrow_schema::{Field, Fields};
use conundrum::ecosystem::db::db_traits::db_field::DatabaseField;
use fake::Dummy;

/// The path on the user's system, divided between the workspace path and the
/// remaining relative path.
#[derive(serde::Serialize, serde::Deserialize, Clone, Debug, specta::Type, Dummy)]
pub struct WorkspaceRelativeStringPath {
    /// The path to the root of the user's workspace
    pub workspace_path: String,
    /// The relative path from the user's workspace root to the file or
    /// directory in question.
    pub relative_path: String,
}

impl DatabaseField for WorkspaceRelativeStringPath {
    fn field_definition(field_key: &'static str, nullable: bool) -> Field {
        Field::new(field_key.to_string(),
                   arrow_schema::DataType::Struct(Fields::from(vec![Field::new("workspace_path",
                                                                               arrow_schema::DataType::Utf8,
                                                                               false),
                                                                    Field::new("relative_path",
                                                                               arrow_schema::DataType::Utf8,
                                                                               false),])),
                   nullable)
    }
}

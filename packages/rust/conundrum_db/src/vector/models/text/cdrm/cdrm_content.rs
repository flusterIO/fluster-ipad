use std::sync::Arc;

use arrow_schema::Field;
use conundrum::ecosystem::db::traits::db_entity::DBSchema;
use conundrum_fs::models::user_workspace::{
    workspace_relative_path::WorkspaceRelativePath, workspace_relative_path_strings::WorkspaceRelativeStringPath,
};
use fake::Dummy;
use serde::{Deserialize, Serialize};

use crate::vector::{
    database::db_traits::{
        db_field::{DatabaseField, DatabaseFieldLarge},
        impls::workspace_relative_path_field::workspace_relative_path_field,
    },
    models::{
        ai::{ai_generated_status::AIGeneratedStatus, ai_interactions::AIInteractions},
        date_time::date_time::DateTime,
        primitives::db_id::DatabaseId,
        taggables::{subject::Subject, tag_list::TagList, taggables::Taggables, topic::Topic},
    },
};

#[derive(Serialize, Deserialize, Clone, Debug, Dummy)]
pub struct CdrmContent {
    pub id: DatabaseId,
    pub title: Option<String>,
    pub content: String,
    pub ai_generated: AIGeneratedStatus,
    pub taggables: Taggables,
    pub ws_path: Option<WorkspaceRelativeStringPath>,
    pub ctime: DateTime,
    pub utime: DateTime,
    pub ai: AIInteractions,
}

pub fn general_text_based_fields(can_extract_title: bool) -> Vec<Arc<Field>> {
    let mut res = vec![Arc::new(DatabaseId::field_definition("id", false)),
                       Arc::new(String::field_definition_large("content", false)),
                       Arc::new(AIGeneratedStatus::field_definition("ai_generated", false)),
                       Arc::new(workspace_relative_path_field("ws_path", true)),
                       Arc::new(DateTime::field_definition("ctime", false)),
                       Arc::new(DateTime::field_definition("utime", false)),
                       Arc::new(AIInteractions::field_definition("ai", false))];
    if can_extract_title {
        res.push(Arc::new(Field::new("title", arrow_schema::DataType::Utf8, true)));
    }
    res
}

impl<'a> DBSchema<'a> for CdrmContent {
    fn arrow_fields(
        )
        -> conundrum::ecosystem::error_handling::db_error::DatabaseResult<Vec<std::sync::Arc<arrow_schema::Field>>>
    {
        Ok(general_text_based_fields(true))
    }
}

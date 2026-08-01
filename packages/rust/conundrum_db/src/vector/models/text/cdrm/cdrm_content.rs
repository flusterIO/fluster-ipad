use conundrum_fs::models::user_workspace::workspace_relative_path::WorkspaceRelativePath;
use serde::{Deserialize, Serialize};

use crate::vector::models::{
    ai::ai_generated_status::AIGeneratedStatus,
    date_time::date_time::DateTime,
    primitives::db_id::DatabaseId,
    taggables::{subject::Subject, tag_list::TagList, taggables::Taggables, topic::Topic},
};

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct CdrmContent {
    pub id: DatabaseId,
    pub title: Option<String>,
    pub content: String,
    pub ai_generated: AIGeneratedStatus,
    pub taggables: Taggables,
    pub ws_path: Option<WorkspaceRelativePath>,
    pub ctime: DateTime,
    pub utime: DateTime,
}

use serde::{Deserialize, Serialize};
use surrealdb::types::SurrealValue;

use crate::vector::models::{
    ai::ai_generated_status::AIGeneratedStatus,
    date_time::date_time::DateTime,
    primitives::db_id::DatabaseId,
    taggables::{subject::Subject, tag::Tag, topic::Topic},
};

#[derive(Serialize, Deserialize, Clone, Debug, SurrealValue)]
pub struct TypstContent {
    pub id: DatabaseId,
    pub title: Option<String>,
    pub body: String,
    pub ai_generated: AIGeneratedStatus,
    pub tags: Vec<Tag>,
    pub topic: Option<Topic>,
    pub subject: Option<Subject>,
    pub fs_path: Option<String>,
    pub ctime: DateTime,
    pub utime: DateTime,
}

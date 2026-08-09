use std::sync::Arc;

use lancedb::arrow::arrow_schema::Schema;
use serde::{Deserialize, Serialize};

use crate::vector::{
    database::db_traits::db_field::DatabaseField,
    models::{
        academic::question::flashcard::flashcard_value::FlashcardValue, date_time::date_time::DateTime,
        primitives::db_id::DatabaseId,
    },
};

pub fn default_empty() -> u32 {
    0
}

#[derive(Clone, Serialize, Deserialize)]
pub struct FlashCardEntity {
    #[serde(default = "DatabaseId::default")]
    pub id: DatabaseId,
    pub question: String,
    pub answer: FlashcardValue,
    pub explanation: Option<String>,
    #[serde(default = "default_empty")]
    pub correct_responses: u32,
    #[serde(default = "default_empty")]
    pub incorrect_responses: u32,
    /// The difficulty field is not optional for AI. AI should always provide an
    /// estimated difficulty score using a scale where Ph.D. level physics
    /// and M.D. level biology is a 100, and elementary math like 2 + 2 is
    /// 0.
    pub difficulty: Option<f32>,
    #[serde(default = "DateTime::new_now")]
    pub ctime: DateTime,
    pub last_access: DateTime,
}

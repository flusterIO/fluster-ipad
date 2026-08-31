use crate::vector::models::academic::question::flashcard::flashcard_value::FlashcardValue;
use conundrum::{
    ecosystem::db::{
        db_traits::{
            db_entity::{DBEntity, DBSchema},
            db_field::DatabaseField,
        },
        tables::DatabaseTable,
    },
    impl_default_crud,
    lifted_models::primitives::{date_time::DateTime, db_id::DatabaseId},
};
use conundrum_macros::DatabaseEntity;
use fake::Dummy;
use serde::{Deserialize, Serialize};
use std::sync::Arc;

pub fn default_empty() -> u32 {
    0
}

#[derive(Clone, Serialize, Deserialize, Dummy, specta::Type, DatabaseEntity)]
#[db(table = DatabaseTable::QAPair)]
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
    #[serde(default = "DateTime::new_now")]
    pub utime: DateTime,
    #[serde(default = "DateTime::new_now")]
    pub last_access: DateTime,
}

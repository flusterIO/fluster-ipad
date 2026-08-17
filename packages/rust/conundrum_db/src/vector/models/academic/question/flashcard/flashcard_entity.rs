use crate::vector::models::academic::question::flashcard::{
    flashcard_entity_partial::FlashCardEntityPartial, flashcard_value::FlashcardValue,
};
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
use fake::Dummy;
use serde::{Deserialize, Serialize};
use std::sync::Arc;

pub fn default_empty() -> u32 {
    0
}

#[derive(Clone, Serialize, Deserialize, Dummy, specta::Type)]
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

impl<'a> DBSchema<'a> for FlashCardEntity {
    fn arrow_fields(
        )
        -> conundrum::ecosystem::error_handling::db_error::DatabaseResult<Vec<std::sync::Arc<arrow_schema::Field>>>
    {
        Ok(vec![Arc::new(DatabaseId::field_definition("id", false)),
                Arc::new(String::field_definition("question", false)),
                FlashcardValue::field_definition("answer", true),
                Arc::new(String::field_definition("explanation", true)),
                Arc::new(u32::field_definition("correct_responses", false)),
                Arc::new(u32::field_definition("incorrect_responses", false)),
                Arc::new(f32::field_definition("difficulty", true)),
                Arc::new(DateTime::field_definition("ctime", false)),
                Arc::new(DateTime::field_definition("utime", false)),
                Arc::new(DateTime::field_definition("last_access", false)),])
    }
}

impl<'a> DBEntity<'a, DatabaseId> for FlashCardEntity {
    type PartialUpdateType = FlashCardEntityPartial;

    fn table() -> conundrum::ecosystem::db::tables::DatabaseTable {
        DatabaseTable::QAPair
    }

    fn merge_keys() -> &'static [&'static str] {
        &["id"]
    }

    fn primary_key() -> &'static str {
        "id"
    }

    fn primary_value(&self) -> DatabaseId {
        self.id.clone()
    }
}

impl_default_crud!(FlashCardEntity, FlashCardEntityPartial, DatabaseId);

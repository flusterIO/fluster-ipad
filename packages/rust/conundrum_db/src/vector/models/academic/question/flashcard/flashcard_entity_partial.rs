use std::sync::Arc;

use conundrum::ecosystem::db::traits::db_entity::DBSchema;
use fake::Dummy;
use serde::{Deserialize, Serialize};

use crate::{
    impl_default_crud,
    vector::{
        database::db_traits::db_field::DatabaseField,
        models::{academic::question::flashcard::flashcard_value::FlashcardValue, primitives::db_id::DatabaseId},
    },
};

#[derive(Clone, Serialize, Deserialize, Dummy, specta::Type)]
pub struct FlashCardEntityPartial {
    pub id: DatabaseId,
    pub question: Option<String>,
    pub answer: Option<FlashcardValue>,
    pub explanation: Option<String>,
    pub correct_responses: Option<u32>,
    pub incorrect_responses: Option<u32>,
    pub difficulty: Option<f32>,
}

impl<'a> DBSchema<'a> for FlashCardEntityPartial {
    fn arrow_fields(
        )
        -> conundrum::ecosystem::error_handling::db_error::DatabaseResult<Vec<std::sync::Arc<arrow_schema::Field>>>
    {
        Ok(vec![Arc::new(DatabaseId::field_definition("id", false)),
                Arc::new(String::field_definition("question", true)),
                FlashcardValue::field_definition("answer", true),
                Arc::new(String::field_definition("explanation", true)),
                Arc::new(u32::field_definition("correct_responses", true)),
                Arc::new(u32::field_definition("incorrect_responses", true)),
                Arc::new(f32::field_definition("difficulty", true)),])
    }
}

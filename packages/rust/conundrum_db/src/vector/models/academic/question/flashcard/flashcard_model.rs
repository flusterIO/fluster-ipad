use conundrum::{
    ecosystem::{db::tables::DatabaseTable, error_handling::db_error::DatabaseResult},
    lifted_models::primitives::{date_time::DateTime, db_id::DatabaseId},
};
use conundrum_macros::DatabaseEntity;
use serde::{Deserialize, Serialize};

use crate::vector::models::{
    academic::question::flashcard::{flashcard_entity::FlashCardEntity, flashcard_value::FlashcardValue},
    taggables::{subject::Subject, tag::Tag, tag_list::TagList, taggables::Taggables, topic::Topic},
};

#[derive(Clone, Deserialize, Debug)]
pub struct FlashCardModelStringAnswerInputData {
    pub question: String,
    pub answer: String,
    /// This is not optional for AI. AI should always produce an explanation.
    pub explanation: Option<String>,
    pub tags: Vec<String>,
    pub subject: Option<String>,
    pub topic: Option<String>,
    /// A subjective difficulty score, probably coming from AI in most cases.
    /// This number must be clamped between 0 and 100 for reliability
    /// between different implementations.
    pub difficulty: Option<f32>,
}

impl Into<FlashCardEntity> for FlashCardModelStringAnswerInputData {
    fn into(self) -> FlashCardEntity {
        FlashCardEntity { id: DatabaseId::new(),
                          question: self.question.clone(),
                          answer: FlashcardValue::Text(self.answer.clone()),
                          explanation: self.explanation.clone(),
                          correct_responses: 0,
                          incorrect_responses: 0,
                          difficulty: self.difficulty.clone(),
                          ctime: DateTime::new_now(),
                          utime: DateTime::new_now(),
                          last_access: DateTime::new_now() }
    }
}

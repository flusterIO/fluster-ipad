use conundrum::ecosystem::{db::tables::DatabaseTable, error_handling::db_error::DatabaseResult};
use serde::{Deserialize, Serialize};

use crate::vector::models::{
    academic::question::flashcard::flashcard_value::FlashcardValue,
    date_time::date_time::DateTime,
    primitives::db_id::DatabaseId,
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

#[derive(Clone, Serialize, Deserialize)]
pub struct FlashCardModel {
    pub id: DatabaseId,
    pub question: String,
    pub answer: FlashcardValue,
    pub explanation: Option<String>,
    pub correct_responses: u32,
    pub incorrect_responses: u32,
    pub tags: TagList,
    pub subject: Option<Subject>,
    pub topic: Option<Topic>,
    /// A subjective difficulty score, probably coming from AI in most cases.
    /// This number must be clamped between 0 and 100 for reliability
    /// between different implementations.
    pub difficulty: Option<f32>,
    pub ctime: DateTime,
    pub last_access: DateTime,
}

impl FlashCardModel {
    pub fn new_with_string_answer(input_data: FlashCardModelStringAnswerInputData) -> FlashCardModel {
        FlashCardModel { id: DatabaseId::new(),
                         question: input_data.question,
                         answer: FlashcardValue::Text(input_data.answer),
                         explanation: input_data.explanation,
                         difficulty: input_data.difficulty,
                         correct_responses: 0,
                         incorrect_responses: 0,
                         tags: TagList::from_strings(input_data.tags),
                         subject: input_data.subject.map(Subject::from),
                         topic: input_data.topic.map(Topic::from),
                         ctime: DateTime::new_now(),
                         last_access: DateTime::new_now() }
    }
}

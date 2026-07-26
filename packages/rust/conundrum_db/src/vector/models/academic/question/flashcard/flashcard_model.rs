use std::sync::Arc;

use arrow_array::{RecordBatch, StringArray};
use conundrum::{
    ecosystem::{
        db::traits::database_field_representable::DatabaseFieldRepresentable,
        error_handling::db_error::{DatabaseError, DatabaseResult},
    },
    parsers::conundrum::logic::{number::conundrum_number::ConundrumNumber, string::conundrum_string::ConundrumString},
};
use serde::{Deserialize, Serialize};

use crate::vector::{
    database::db_types::db_entity::DBEntity,
    models::{
        date_time::date_time::DateTime,
        primitives::db_id::DatabaseId,
        taggables::{subject::Subject, tag::Tag, topic::Topic},
    },
};

#[derive(Clone, Deserialize)]
pub struct FlashCardModelInputData<T> {
    pub question: String,
    pub answer: T,
    /// This is not optional for AI. AI should always produce an explanation.
    pub explanation: Option<String>,
    pub tags: Vec<Tag>,
    pub subject: Option<Subject>,
    pub topic: Option<Topic>,
    /// A subjective difficulty score, probably coming from AI in most cases.
    /// This number must be clamped between 0 and 100 for reliability
    /// between different implementations.
    pub difficulty: Option<usize>,
}

#[derive(Clone, Serialize, Deserialize)]
pub struct FlashCardModel<T> {
    pub id: DatabaseId,
    pub question: String,
    pub answer: T,
    pub explanation: Option<String>,
    pub correct_responses: u32,
    pub incorrect_responses: u32,
    pub tags: Vec<Tag>,
    pub subject: Option<Subject>,
    pub topic: Option<Topic>,
    /// A subjective difficulty score, probably coming from AI in most cases.
    /// This number must be clamped between 0 and 100 for reliability
    /// between different implementations.
    pub difficulty: Option<usize>,
    pub ctime: DateTime,
    pub last_access: DateTime,
}

impl<T> FlashCardModel<T> {
    fn new(input_data: FlashCardModelInputData<T>) -> FlashCardModel<T> {
        FlashCardModel { id: DatabaseId::new(),
                         question: input_data.question,
                         answer: input_data.answer,
                         explanation: input_data.explanation,
                         difficulty: input_data.difficulty,
                         correct_responses: 0,
                         incorrect_responses: 0,
                         tags: input_data.tags,
                         subject: input_data.subject,
                         topic: input_data.topic,
                         ctime: DateTime::new_now(),
                         last_access: DateTime::new_now() }
    }
}

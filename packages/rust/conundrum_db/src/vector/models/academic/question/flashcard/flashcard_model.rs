use conundrum::ecosystem::{db::tables::DatabaseTable, error_handling::db_error::DatabaseResult};
use serde::{Deserialize, Serialize};
use surrealdb_types::RecordId;

use crate::vector::{
    database::{db::ArcMutexDB, db_traits::pure_model_instance::PureModelInstanceMethods},
    models::{
        academic::question::flashcard::{flashcard_value::FlashcardValue, pure_flashcard::PureFlashcard},
        date_time::date_time::DateTime,
        primitives::db_id::DatabaseId,
        taggables::{subject::Subject, tag::Tag, topic::Topic},
    },
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
    pub tags: Vec<Tag>,
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
        FlashCardModel { id: DatabaseId::new(DatabaseTable::QAPair),
                         question: input_data.question,
                         answer: FlashcardValue::Text(input_data.answer),
                         explanation: input_data.explanation,
                         difficulty: input_data.difficulty,
                         correct_responses: 0,
                         incorrect_responses: 0,
                         tags: input_data.tags.iter().map(|t| Tag::from(t.clone())).collect::<Vec<Tag>>(),
                         subject: input_data.subject.map(Subject::from),
                         topic: input_data.topic.map(Topic::from),
                         ctime: DateTime::new_now(),
                         last_access: DateTime::new_now() }
    }

    pub async fn upsert_self(&self, db: &ArcMutexDB) -> DatabaseResult<RecordId> {
        for t in self.tags.iter() {
            t.upsert_self(db).await?;
        }
        if let Some(subject) = &self.subject {
            subject.upsert_self(db).await?;
        }
        if let Some(topic) = &self.topic {
            topic.upsert_self(db).await?;
        }
        let r = PureFlashcard::from(self.clone());
        let record_id: RecordId = r.upsert_self(db).await?;
        Ok(record_id)
    }
}

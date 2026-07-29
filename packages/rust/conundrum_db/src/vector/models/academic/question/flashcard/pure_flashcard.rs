use conundrum::{
    ecosystem::{db::tables::DatabaseTable, error_handling::db_error::DatabaseError},
    parsers::conundrum::logic::number::conundrum_float::ConundrumFloat,
};
use indoc::formatdoc;
use serde::Serialize;
use surrealdb_types::{RecordId, SurrealValue};

use crate::vector::{
    database::{
        db_traits::{
            database_field::{DatabaseField, OptionalDatabaseField},
            pure_model_instance::PureModelInstanceMethods,
            pure_model_static::PureModelStaticMethods,
        },
        primitive_field_schema_generators::string_field_def_generator::{
            optional_clamped_float_field_definition, optional_string_field_definition, string_field_definition,
            unsigned_int_field_definition,
        },
    },
    models::{
        academic::question::flashcard::{
            flashcard_model::{FlashCardModel, FlashCardModelStringAnswerInputData},
            flashcard_value::FlashcardValue,
        },
        date_time::date_time::DateTime,
        primitives::db_id::DatabaseId,
    },
};

#[derive(Clone, Debug, Serialize, SurrealValue)]
pub struct PureFlashcard {
    pub id: DatabaseId,
    pub question: String,
    /// Only valid type currently supported is a string, but this is in place to
    /// make room for numerical equality comparisons in the future.
    pub answer: FlashcardValue,
    /// ## AI
    /// This field is not optional for AI. Capable AI models shouold always
    /// provide an explanation.
    pub explanation: Option<String>,
    /// A number clampped between 0 and 100 indicating the difficulty of the
    /// question.
    ///
    /// ## AI
    ///
    /// AI should interpret this subjective scale with 100 being M.D. level
    /// biology or Ph.D. level physics, and 0 being elementary school
    /// mathematics like 2+2.
    pub difficulty: Option<ConundrumFloat>,
    pub correct_responses: u32,
    pub incorrect_responses: u32,
    pub ctime: DateTime,
    /// This field is updated each time the question is retrieved *alone*. This
    /// won't be a perfect counter, but it should indicate which questions
    /// have been accessed more than others with enough accuracy to build
    /// features around it.
    pub last_access: Option<DateTime>,
}

impl From<FlashCardModel> for PureFlashcard {
    fn from(value: FlashCardModel) -> Self {
        PureFlashcard { id: DatabaseId::new(DatabaseTable::QAPair),
                        question: value.question.clone(),
                        answer: value.answer.clone(),
                        explanation: value.explanation.clone(),
                        difficulty: value.difficulty.map(ConundrumFloat::from),
                        correct_responses: 0,
                        incorrect_responses: 0,
                        ctime: DateTime::new_now(),
                        last_access: None }
    }
}

impl From<FlashCardModelStringAnswerInputData> for PureFlashcard {
    fn from(value: FlashCardModelStringAnswerInputData) -> Self {
        PureFlashcard { id: DatabaseId::new(DatabaseTable::QAPair),
                        question: value.question.clone(),
                        answer: FlashcardValue::Text(value.answer.clone()),
                        explanation: value.explanation.clone(),
                        difficulty: value.difficulty.map(ConundrumFloat::from),
                        correct_responses: 0,
                        incorrect_responses: 0,
                        ctime: DateTime::new_now(),
                        last_access: None }
    }
}

impl PureModelStaticMethods for PureFlashcard {
    fn table() -> conundrum::ecosystem::db::tables::DatabaseTable {
        DatabaseTable::QAPair
    }

    fn schema() -> String {
        let tbl = Self::table();
        formatdoc! {"
        {}
        {}
        {}
        {}
        {}
        {}
        {}
        {}
            ", string_field_definition("question", &tbl), string_field_definition("answer", &tbl), optional_string_field_definition("explanation", &tbl), optional_clamped_float_field_definition("difficulty", &tbl, 0, 100), unsigned_int_field_definition("correct_responses", &tbl), unsigned_int_field_definition("incorrect_responses", &tbl), DateTime::field_definition("ctime", &tbl), DateTime::optional_field_definition("last_access", &tbl)}
    }
}

impl PureModelInstanceMethods for PureFlashcard {
    async fn upsert_self(
        &self,
        db: &crate::vector::database::db::ArcMutexDB)
        -> conundrum::ecosystem::error_handling::db_error::DatabaseResult<surrealdb_types::RecordId> {
        let db = db.clone().lock_owned().await;
        let value = db.create(self.id.0.clone())
                      .content(self.clone())
                      .await
                      .map_err(|e| DatabaseError::DatabaseError { source: Some(e) })?
                      .ok_or(DatabaseError::DatabaseError { source: None })?;
        drop(db);
        let deserialized = RecordId::from_value(value).map_err(|e| {
                                                          log::error!("Error: {:?}", e);
                                                          DatabaseError::SerializationError
                                                      })?;
        Ok(deserialized)
    }
}

#[cfg(test)]
mod tests {
    use crate::vector::{
        database::db::get_database,
        models::academic::question::flashcard::flashcard_model::FlashCardModelStringAnswerInputData,
    };

    use super::*;

    #[tokio::test]
    async fn saves_pure_flashcards() {
        let test_data = include_str!("../../../../../../tests/seed_questions.json");
        let deserialized: Vec<FlashCardModelStringAnswerInputData> =
            serde_json::from_str(test_data).expect("Successfully deserializes test questions");
        let db = get_database().await.expect("Gets database without throwing an error.");
        for t in deserialized {
            let flashcard = FlashCardModel::new_with_string_answer(t);
            flashcard.upsert_self(db).await.expect("Saves flashcard without throwing an error");
        }
    }
}

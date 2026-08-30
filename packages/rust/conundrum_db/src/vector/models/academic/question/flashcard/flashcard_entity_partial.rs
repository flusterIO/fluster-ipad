use std::sync::Arc;

use conundrum::{
    ecosystem::db::db_traits::{db_entity::DBSchema, db_field::DatabaseField},
    lifted_models::primitives::db_id::DatabaseId,
};
use conundrum_macros::DBSchema;
use fake::Dummy;
use serde::{Deserialize, Serialize};

use crate::vector::models::academic::question::flashcard::flashcard_value::FlashcardValue;

#[derive(Clone, Serialize, Deserialize, Dummy, specta::Type, DBSchema)]
pub struct FlashCardEntityPartial {
    pub id: DatabaseId,
    pub question: Option<String>,
    pub answer: Option<FlashcardValue>,
    pub explanation: Option<String>,
    pub correct_responses: Option<u32>,
    pub incorrect_responses: Option<u32>,
    pub difficulty: Option<f32>,
}

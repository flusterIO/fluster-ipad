use serde::{Deserialize, Serialize};
use surrealdb::types::{Number, SurrealValue};

#[derive(Clone, Serialize, Deserialize, SurrealValue, Debug)]
pub enum FlashcardValue {
    Numeric(Number),
    Text(String),
}

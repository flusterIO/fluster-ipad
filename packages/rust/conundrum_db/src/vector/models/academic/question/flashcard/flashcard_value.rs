use conundrum::ecosystem::error_handling::db_error::DatabaseError;
use serde::{Deserialize, Serialize};
use std::str::FromStr;
use surrealdb::types::{Number, SurrealValue};

#[derive(Clone, Serialize, Deserialize, SurrealValue, Debug)]
pub enum FlashcardValue {
    Numeric(Number),
    Text(String),
}

impl From<f64> for FlashcardValue {
    fn from(value: f64) -> Self {
        Self::Numeric(Number::from_float(value))
    }
}

impl FromStr for FlashcardValue {
    type Err = DatabaseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(Self::Text(s.to_string()))
    }
}

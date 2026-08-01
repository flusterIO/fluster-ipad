use conundrum::{
    ecosystem::error_handling::db_error::DatabaseError,
    parsers::conundrum::logic::number::{conundrum_float::ConundrumFloat, conundrum_int::ConundrumInt},
};
use lancedb::arrow::arrow_schema::Field;
use serde::{Deserialize, Serialize};
use std::str::FromStr;

use crate::vector::database::db_traits::db_field::DatabaseField;

#[derive(Clone, Serialize, Deserialize, Debug)]
pub enum FlashcardValue {
    Float(ConundrumFloat),
    Int(ConundrumInt),
    Text(String),
}

impl From<i64> for FlashcardValue {
    fn from(value: i64) -> Self {
        let n: ConundrumInt = value.into();
        Self::Int(n)
    }
}

impl From<f64> for FlashcardValue {
    fn from(value: f64) -> Self {
        let n: ConundrumFloat = value.into();
        Self::Float(n)
    }
}

impl From<String> for FlashcardValue {
    fn from(value: String) -> Self {
        Self::Text(value)
    }
}

impl FromStr for FlashcardValue {
    type Err = DatabaseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(Self::Text(s.to_string()))
    }
}

impl DatabaseField for FlashcardValue {
    /// This will only work for strings for now until we can get online and
    /// handle the union schema. Just pretend like the union schema is already
    /// in place.
    fn field_definition(field_key: &'static str, nullable: bool) -> lancedb::arrow::arrow_schema::Field {
        Field::new(field_key, lancedb::arrow::arrow_schema::DataType::Utf8, nullable)
    }
}

use conundrum::ecosystem::error_handling::db_error::DatabaseError;
use serde::{Deserialize, Serialize};
use std::str::FromStr;
use surrealdb::types::{Number, SurrealValue};

use crate::vector::database::db_traits::database_field::DatabaseField;

#[derive(Clone, Serialize, Deserialize, Debug)]
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

impl SurrealValue for FlashcardValue {
    fn kind_of() -> surrealdb_types::Kind {
        surrealdb_types::Kind::Either(vec![surrealdb_types::Kind::Number, surrealdb_types::Kind::String,])
    }

    fn into_value(self) -> surrealdb_types::Value {
        match self {
            Self::Text(s) => surrealdb_types::Value::String(s),
            Self::Numeric(n) => surrealdb_types::Value::Number(n),
        }
    }

    fn from_value(value: surrealdb_types::Value) -> Result<Self, surrealdb::Error>
        where Self: Sized {
        if let Some(n) = value.as_number() {
            Ok(FlashcardValue::Numeric(*n))
        } else if let Some(s) = value.as_string() {
            Ok(FlashcardValue::Text(s.clone()))
        } else {
            Err(surrealdb_types::Error::thrown("Invalid FlashcardValue type.".to_string()))
        }
    }
}

impl DatabaseField for FlashcardValue {
    fn field_definition(field_key: &'static str, table: &conundrum::ecosystem::db::tables::DatabaseTable) -> String {
        format!("DEFINE FIELD IF NOT EXISTS {} ON {} TYPE number|string;", field_key, table)
    }
}

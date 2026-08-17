use conundrum::ecosystem::{
    db::db_traits::db_field::{DatabaseField, DatabaseFieldRepresentation},
    error_handling::db_error::{DatabaseError, DatabaseResult},
};
use fake::{Dummy, Fake, Faker};
use lancedb::arrow::arrow_schema::Field;
use serde::{Deserialize, Serialize};
use std::{str::FromStr, sync::Arc};

use crate::vector::models::utility::generic_value::GenericValue;

#[derive(Clone, Serialize, Deserialize, Debug, specta::Type)]
pub enum FlashcardValue {
    Float(f32),
    Int(i32),
    Text(String),
}

impl Dummy<Faker> for FlashcardValue {
    fn dummy_with_rng<R: fake::rand::prelude::RngExt + ?Sized>(config: &Faker, rng: &mut R) -> Self {
        // TODO: Improve this and actually return a genuinely random value.
        let s: String = Faker.fake();
        FlashcardValue::Text(s)
    }
}

impl From<i32> for FlashcardValue {
    fn from(value: i32) -> Self {
        Self::Int(value)
    }
}

impl From<f32> for FlashcardValue {
    fn from(value: f32) -> Self {
        Self::Float(value)
    }
}

impl From<i64> for FlashcardValue {
    fn from(value: i64) -> Self {
        Self::Int(value as i32)
    }
}

impl From<f64> for FlashcardValue {
    fn from(value: f64) -> Self {
        Self::Float(value as f32)
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
        if let Ok(j) = serde_json::from_str::<GenericValue<String>>(s) {
            Ok(Self::Text(j.value))
        } else if let Ok(k) = serde_json::from_str::<GenericValue<f32>>(s) {
            Ok(Self::Float(k.value))
        } else if let Ok(l) = serde_json::from_str::<GenericValue<i32>>(s) {
            Ok(Self::Int(l.value))
        } else {
            Ok(Self::Text(s.to_string()))
        }
    }
}

impl DatabaseFieldRepresentation<DatabaseResult<String>> for FlashcardValue {
    fn to_db_representation(&self) -> DatabaseResult<String> {
        match self {
            Self::Int(n) => {
                let x = GenericValue { value: *n };
                let json_string = serde_json::to_string(&x).map_err(|e| {
                                                               log::error!("Error: {:?}", e);
                                                               DatabaseError::SerializationError
                                                           })?;
                Ok(json_string)
            }
            Self::Float(n) => {
                let x = GenericValue { value: *n };
                let json_string = serde_json::to_string(&x).map_err(|e| {
                                                               log::error!("Error: {:?}", e);
                                                               DatabaseError::SerializationError
                                                           })?;
                Ok(json_string)
            }
            Self::Text(s) => {
                let x = GenericValue { value: s.to_string() };
                let json_string = serde_json::to_string(&x).map_err(|e| {
                                                               log::error!("Error: {:?}", e);
                                                               DatabaseError::SerializationError
                                                           })?;
                Ok(json_string)
            }
        }
    }
}

impl DatabaseField<Arc<Field>> for FlashcardValue {
    fn field_definition(field_key: &'static str, nullable: bool) -> Arc<lancedb::arrow::arrow_schema::Field> {
        let r = String::field_definition(field_key, nullable);
        Arc::new(r)
    }
}
